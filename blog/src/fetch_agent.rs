pub use crate::cache::{Cache, Load};
use crate::{posts::PostList, utils::Page};
use failure::Error;
use std::time::{SystemTime, UNIX_EPOCH};
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
        let mut url = if target == Page::Posts {
            "posts.json".into()
        } else {
            target.url()
        };
        // avoid cache
        url.push('?');
        // get unix timestamp
        let now = SystemTime::now();
        let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("time wrap!");
        url.push_str(&since_the_epoch.as_secs().to_string());
        let cb = self.link.callback(|x| x);
        let req = Request::get(url)
            .header("Cache-Control", "max-age=120")
            .body(Nothing)
            .unwrap();
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
            self.link.respond(who, msg.clone());
        }
        self.task = None;
    }

    fn handle_input(&mut self, input: Self::Input, who: HandlerId) {
        self.who = Some(who);
        if let Some(cc) = self.cache.get(&input) {
            // cache response
            self.link.respond(who, cc);
        } else {
            // only load when no cache
            self.load(input);
        }
    }
}
