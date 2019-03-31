use crate::posts::PostList;
use crate::utils::Page;
use failure::Error;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use yew::format::Nothing;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::worker::*;
use yew::Callback;

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub enum Load {
    Page(String),
    PostList(PostList),
}

impl Transferable for Load {}

pub struct FetchAgent {
    web: FetchService,
    task: Option<FetchTask>,
    link: AgentLink<FetchAgent>,
    who: Option<HandlerId>,
}

impl FetchAgent {
    pub fn post_list_handle(&self, cb: Callback<Load>) -> impl Fn(Response<Result<String, Error>>) {
        move |res: Response<Result<String, Error>>| {
            let (meta, body) = res.into_parts();
            if meta.status.is_success() {
                if let Ok(payload) = body {
                    let list: PostList = serde_json::from_str(payload.as_str()).unwrap();
                    cb.emit(Load::PostList(list));
                }
            }
        }
    }

    pub fn page_handle(&self, cb: Callback<Load>) -> impl Fn(Response<Result<String, Error>>) {
        move |res: Response<Result<String, Error>>| {
            let (meta, body) = res.into_parts();
            if meta.status.is_success() {
                if let Ok(payload) = body {
                    cb.emit(Load::Page(payload));
                }
            }
        }
    }

    pub fn load(&mut self, target: Page) {
        let url = if target == Page::Posts {
            "post.json".into()
        } else {
            target.url()
        };
        let cb = self.link.send_back(|x| x);
        let req = Request::get(url).body(Nothing).unwrap();
        let task = match target {
            Page::Posts => self.web.fetch(req, self.post_list_handle(cb).into()),
            _ => self.web.fetch(req, self.page_handle(cb).into()),
        };
        self.task = Some(task);
    }
}

impl Agent for FetchAgent {
    type Reach = Context;
    type Message = Load;
    type Input = Page;
    type Output = Load;

    fn create(link: AgentLink<Self>) -> Self {
        FetchAgent {
            link,
            web: FetchService::new(),
            task: None,
            who: None,
        }
    }

    fn update(&mut self, msg: Self::Message) {
        if let Some(who) = self.who {
            self.link.response(who, msg.clone());
        }
        self.task = None;
    }

    fn handle(&mut self, input: Self::Input, who: HandlerId) {
        self.who = Some(who);
        self.load(input);
    }
}
