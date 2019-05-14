pub use crate::cache::{Cache, Load};
use crate::{posts::PostList, utils::Page};
use failure::Error;
use yew::{
    format::Nothing,
    services::fetch::{FetchService, FetchTask, Request, Response},
    worker::*,
    Callback,
};

pub struct FetchAgent {
    cache: Cache,
    current_target: Option<Page>,
    link: AgentLink<FetchAgent>,
    task: Option<FetchTask>,
    web: FetchService,
    who: Option<HandlerId>,
}

impl FetchAgent {
    pub fn post_list_handle(&self, cb: Callback<Load>) -> impl Fn(Response<Result<String, Error>>) {
        move |res: Response<Result<String, Error>>| {
            let (meta, body) = res.into_parts();
            if meta.status.is_success() {
                if let Ok(payload) = body {
                    let list: PostList = serde_json::from_str(payload.as_str()).unwrap();
                    cb.emit(Load::Posts(list));
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
            "posts.json".into()
        } else {
            target.url()
        };
        let cb = self.link.send_back(|x| x);
        let req = Request::get(url).body(Nothing).unwrap();
        let task = match &target {
            Page::Posts => self.web.fetch(req, self.post_list_handle(cb).into()),
            _ => self.web.fetch(req, self.page_handle(cb).into()),
        };
        self.current_target = Some(target);
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
            cache: Cache::new(),
            current_target: None,
        }
    }

    fn update(&mut self, msg: Self::Message) {
        self.cache.set(&self.current_target.take().unwrap(), &msg);
        if let Some(who) = self.who {
            self.link.response(who, msg.clone());
        }
        self.task = None;
    }

    fn handle(&mut self, input: Self::Input, who: HandlerId) {
        self.who = Some(who);
        if let Some(cc) = self.cache.get(&input) {
            // cache response
            self.link.response(who, cc);
        }
        self.load(input);
    }
}
