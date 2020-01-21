pub use crate::cache::{Cache, Load};
use crate::{posts::PostList, utils::Page};
use serde::{Deserialize, Serialize};
use yew::worker::*;

pub enum FetchResult {
    Cacheable(Load, Page, u32),
    Uncacheable(String),
}

#[derive(Clone, Deserialize, Serialize)]
pub enum FetchRequest {
    Cacheable(Page),
    Uncacheable(String),
}

impl From<Page> for FetchRequest {
    fn from(page: Page) -> FetchRequest {
        FetchRequest::Cacheable(page)
    }
}

impl FetchRequest {
    fn uri(&self) -> String {
        match self {
            Self::Cacheable(page) => page.url(),
            Self::Uncacheable(uri) => uri.clone(),
        }
    }
}

impl FetchRequest {
    pub fn fill(self, res: String, update_id: u32) -> FetchResult {
        match self {
            FetchRequest::Cacheable(Page::Posts) => {
                let list: PostList = serde_json::from_str(&res).unwrap();
                FetchResult::Cacheable(Load::Posts(list), Page::Posts, update_id)
            }
            FetchRequest::Uncacheable(_uri) => FetchResult::Uncacheable(res),
            FetchRequest::Cacheable(page) => {
                FetchResult::Cacheable(Load::Page(res), page, update_id)
            }
        }
    }
}

pub struct FetchAgent {
    cache: Cache,
    link: AgentLink<FetchAgent>,
    who: Option<HandlerId>,
    base: String,
    update_id: u32,
}

mod fetch {
    use wasm_bindgen::JsValue;
    use web_sys::{window, Response};

    pub async fn get(uri: &str) -> Result<Option<String>, JsValue> {
        let window = window().unwrap();
        let js_promise = window.fetch_with_str(&uri);
        let response: Response = wasm_bindgen_futures::JsFuture::from(js_promise)
            .await?
            .into();
        let res = wasm_bindgen_futures::JsFuture::from(response.text().unwrap())
            .await?
            .as_string();
        Ok(res)
    }
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
            if let Ok(Some(res)) = fetch::get(&uri).await {
                let fetch_result = target.fill(res, update_id);
                cb.emit(fetch_result);
            }
        };
        wasm_bindgen_futures::spawn_local(future);
    }
}

impl Agent for FetchAgent {
    type Reach = Context;
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
        if let FetchResult::Cacheable(msg, page, update_id) = msg {
            self.cache.set(&page, &msg);
            if self.update_id > update_id {
                // over date
                return;
            }
            if let Some(who) = self.who {
                self.link.respond(who, msg);
            }
        }
    }

    fn handle_input(&mut self, input: Self::Input, who: HandlerId) {
        self.who = Some(who);
        if let FetchRequest::Cacheable(ref page) = input {
            if let Some(cc) = self.cache.get(&page) {
                // cache response
                self.link.respond(who, cc);
            } else {
                // only load when no cache
                self.fetch(input);
            }
        } else {
            self.fetch(input);
        }
    }
}

#[cfg(test)]
mod fetch_agent_tests {
    use super::*;
    #[test]
    fn random_link_test() {
        let mut url = "/index.md".to_string();
        let mid = url.len() + 1;
        FetchAgent::random_link(&mut url);
        let (front, end) = url.split_at(mid);
        assert_eq!(front, "/index.md?");
        let _end_num: u8 = end.parse().unwrap();
    }
}
