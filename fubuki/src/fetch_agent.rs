use crate::Route;
use fubuki_types::PostList;
use gloo_net::http::Request;
use std::collections::HashMap;
use yew_agent::{Agent, AgentLink, Context, HandlerId};

#[derive(Clone)]
pub enum Response {
    Posts(PostList),
    Page(String),
}

/// (response, id)
#[derive(Clone)]
pub struct FetchResult(Response, u32);

#[derive(Clone)]
pub enum FetchRequest {
    Inner(Route),
}

impl From<Route> for FetchRequest {
    fn from(page: Route) -> FetchRequest {
        FetchRequest::Inner(page)
    }
}

fn route_to_url(route: &Route) -> String {
    match route {
        Route::Posts => "/posts.yml".to_owned(),
        Route::Post { id } => format!("/public/post/{}.md", id),
        Route::Main => "/public/index.md".to_owned(),
        Route::About => "/public/about.md".to_owned(),
        Route::Links => "/public/links.md".to_owned(),
    }
}

impl FetchRequest {
    fn uri(&self) -> String {
        match self {
            FetchRequest::Inner(route) => route_to_url(route),
        }
    }

    pub fn fill(self, res: String, update_id: u32) -> serde_yaml::Result<FetchResult> {
        let fetch_result = match self {
            FetchRequest::Inner(Route::Posts) => {
                let list: PostList = serde_yaml::from_str(&res)?;
                FetchResult(Response::Posts(list), update_id)
            }
            FetchRequest::Inner(_) => FetchResult(Response::Page(res), update_id),
        };
        Ok(fetch_result)
    }
}

pub struct FetchAgent {
    link: AgentLink<FetchAgent>,
    update_id: u32,
    who: HashMap<u32, HandlerId>,
}

impl FetchAgent {
    fn get_id(&mut self) -> u32 {
        self.update_id += 1;
        self.update_id
    }

    fn fetch(&mut self, target: FetchRequest, update_id: u32) {
        let uri = target.uri();
        log::debug!("fetch {}", &uri);
        let future = async move {
            let res = Request::get(&uri).send().await;
            let res = res.map_err(|e| log::error!("fetch error: {}", e)).ok()?;
            let text = res.text().await.map_err(|e| log::error!("parse error: {}", e)).ok()?;

            target.fill(text, update_id).ok()
        };
        self.link.send_future(future);
    }
}

impl Agent for FetchAgent {
    type Reach = Context<Self>;
    type Message = Option<FetchResult>;
    type Input = FetchRequest;
    type Output = Response;

    fn create(link: AgentLink<Self>) -> Self {
        FetchAgent {
            link,
            who: HashMap::new(),
            update_id: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) {
        let msg = if let Some(msg) = msg { msg } else { return };
        let FetchResult(msg, update_id) = msg;
        if let Some(who) = self.who.remove(&update_id) {
            self.link.respond(who, msg);
        }
    }

    fn handle_input(&mut self, input: Self::Input, who: HandlerId) {
        let update_id = self.get_id();
        let dup_key = self.who.insert(update_id, who).is_some();
        assert!(!dup_key, "should never have dup key!");
        self.fetch(input, update_id);
    }
}
