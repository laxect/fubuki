pub use crate::cache::{Cache, Load};
use crate::{posts::PostList, utils::Page};
use yew::worker::{Agent, AgentLink, Context, HandlerId};

pub mod fetch {
    use wasm_bindgen::JsValue;
    use wasm_bindgen_futures::JsFuture;
    use web_sys::{window, RequestCache, RequestInit, RequestMode, Response};

    pub async fn get(uri: &str) -> Result<String, JsValue> {
        let mut fetch_config = RequestInit::new();
        fetch_config.mode(RequestMode::Cors);
        fetch_config.cache(RequestCache::Reload);
        // get windows object
        let window = window().ok_or_else(|| JsValue::from_str("open window failed"))?;
        let response = window.fetch_with_str_and_init(uri, &fetch_config);
        let response: Response = JsFuture::from(response).await?.into();
        let res = JsFuture::from(response.text().unwrap())
            .await?
            .as_string()
            .ok_or_else(|| JsValue::from_str("no body"))?;
        Ok(res)
    }
}

#[derive(Clone)]
pub struct FetchResult(Load, Page, u32);

#[derive(Clone)]
pub struct FetchRequest(Page);

impl From<Page> for FetchRequest {
    fn from(page: Page) -> FetchRequest {
        FetchRequest(page)
    }
}

impl FetchRequest {
    fn uri(&self) -> String {
        self.0.url()
    }

    pub fn fill(self, res: String, update_id: u32) -> serde_yaml::Result<FetchResult> {
        let fetch_result = match self {
            FetchRequest(Page::Posts) => {
                let list: PostList = serde_yaml::from_str(&res)?;
                FetchResult(Load::Posts(list), Page::Posts, update_id)
            }
            FetchRequest(page) => FetchResult(Load::Page(res), page, update_id),
        };
        Ok(fetch_result)
    }
}

pub struct FetchAgent {
    cache: Cache,
    link: AgentLink<FetchAgent>,
    who: Option<HandlerId>,
    base: String,
    update_id: u32,
}

impl FetchAgent {
    fn get_id(&mut self) -> u32 {
        self.update_id += 1;
        self.update_id
    }

    fn random_link(url: &mut String) {
        let mut end = [0u8; 1];
        getrandom::getrandom(&mut end).unwrap();
        let append = format!("?{}", end[0]);
        url.push_str(&append);
    }

    fn get_uri(&self, target: &FetchRequest) -> String {
        let mut uri = target.uri();
        if uri.starts_with("http") {
            return uri;
        }
        uri.insert_str(0, &self.base);
        Self::random_link(&mut uri);
        uri
    }

    fn fetch(&mut self, target: FetchRequest) {
        let uri = self.get_uri(&target);
        let cb = self.link.callback(|x| x);
        let update_id = self.get_id();
        let future = async move {
            if let Ok(res) = fetch::get(&uri).await {
                let _ = target
                    .fill(res, update_id)
                    .map(|fetch_result| cb.emit(fetch_result));
            }
        };
        wasm_bindgen_futures::spawn_local(future);
    }
}

impl Agent for FetchAgent {
    type Reach = Context<Self>;
    type Message = FetchResult;
    type Input = FetchRequest;
    type Output = Load;

    fn create(link: AgentLink<Self>) -> Self {
        let base = web_sys::window().unwrap().location().origin().unwrap();
        FetchAgent {
            link,
            who: None,
            cache: Cache::new(),
            base,
            update_id: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) {
        let FetchResult(msg, page, update_id) = msg;
        self.cache.set(&page, &msg);
        if self.update_id > update_id {
            // over date
            return;
        }
        if let Some(who) = self.who {
            self.link.respond(who, msg);
        }
    }

    fn handle_input(&mut self, input: Self::Input, who: HandlerId) {
        self.who = Some(who);
        let FetchRequest(ref page) = input;
        if let Some(cc) = self.cache.get(page) {
            // cache response
            self.link.respond(who, cc);
        } else {
            // only load when no cache
            self.fetch(input);
        }
    }
}
