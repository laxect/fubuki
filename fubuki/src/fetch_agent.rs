use crate::Route;
use fubuki_types::PostList;
use yew_agent::{Agent, AgentLink, Context, HandlerId};

#[derive(Clone)]
pub enum Load {
    Posts(PostList),
    Page(String),
}

#[derive(Clone)]
pub struct FetchResult(Load, Route, u32);

#[derive(Clone)]
pub struct FetchRequest(pub Route);

impl From<Route> for FetchRequest {
    fn from(page: Route) -> FetchRequest {
        FetchRequest(page)
    }
}

fn route_to_url(route: &Route) -> String {
    match route {
        Route::Posts => "/public/posts.yml".to_owned(),
        Route::Post { id } => format!("/public/post/{}.md", id),
        Route::Main => "/public/index.md".to_owned(),
        Route::About => "/public/about.md".to_owned(),
        Route::Links => "/public/links.md".to_owned(),
    }
}

impl FetchRequest {
    fn uri(&self) -> String {
        route_to_url(&self.0)
    }

    pub fn fill(self, res: String, update_id: u32) -> serde_yaml::Result<FetchResult> {
        let fetch_result = match self {
            FetchRequest(Route::Posts) => {
                let list: PostList = serde_yaml::from_str(&res)?;
                FetchResult(Load::Posts(list), Route::Posts, update_id)
            }
            FetchRequest(page) => FetchResult(Load::Page(res), page, update_id),
        };
        Ok(fetch_result)
    }
}

pub struct FetchAgent {
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

    fn get_uri(&self, target: &FetchRequest) -> String {
        let mut uri = target.uri();
        if uri.starts_with("http") {
            return uri;
        }
        uri.insert_str(0, &self.base);
        uri
    }

    fn fetch(&mut self, target: FetchRequest) {
        let uri = self.get_uri(&target);
        let cb = self.link.callback(|x| x);
        let update_id = self.get_id();
        let future = async move {
            let res = gloo_net::http::Request::get(&uri)
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            target
                .fill(res, update_id)
                .map(|fetch_result| cb.emit(fetch_result))
                .ok();
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
        let base = gloo_utils::window().location().origin().unwrap();
        FetchAgent {
            link,
            who: None,
            base,
            update_id: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) {
        let FetchResult(msg, _page, update_id) = msg;
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
        let FetchRequest(ref _page) = input;
        self.fetch(input);
    }
}
