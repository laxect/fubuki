use crate::Page;
use serde_derive::{Deserialize, Serialize};
use stdweb::{
    unstable::TryFrom,
    web::{
        document, event::PopStateEvent, window, EventListenerHandle, History, IEventTarget,
        Location,
    },
    Value,
};
use yew::worker::*;

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub enum Request {
    Where,
    Goto(Page),
}

impl Transferable for Request {}

pub struct Router {
    history: History,
    location: Location,
    link: AgentLink<Router>,
    event_listener: Option<EventListenerHandle>,
    who: Option<HandlerId>,
}

impl Router {
    pub fn register_callback(&mut self) {
        let cb = self.link.send_back(|x| x);
        self.event_listener = Some(window().add_event_listener(move |event: PopStateEvent| {
            let state_value: Value = event.state();
            if let Ok(state) = String::try_from(state_value) {
                if let Ok(page) = Page::try_from(state) {
                    cb.emit(page);
                }
            } else {
                eprintln!("Nothing farther back in history, not calling routing callback.");
            }
        }));
    }

    fn set_route(&mut self, page: Page) {
        let mut route = page.value();
        route.insert_str(0, "/");
        self.history
            .push_state(page.value(), &page.title(), Some(route.as_str()));
        document().set_title(&page.title());
    }

    fn get_path(&self) -> Page {
        let mut path = self.location.pathname().unwrap();
        if path.starts_with('/') {
            path = path.replacen("/", "", 1);
        }
        if path.ends_with('/') {
            path.pop();
        }
        Page::try_from(path).unwrap()
    }

    fn replace_path(&mut self, page: Page) {
        let mut route = page.value();
        route.insert_str(0, "/");
        let _ = self
            .history
            .replace_state(page.value(), &page.title(), Some(route.as_str()));
        document().set_title(&page.title());
    }
}

impl Agent for Router {
    type Reach = Context;
    type Message = Page;
    type Input = Request;
    type Output = Page;

    fn create(link: AgentLink<Self>) -> Router {
        let location = window()
            .location()
            .expect("browser does not support location API");
        let mut router = Router {
            link,
            history: window().history(),
            location,
            event_listener: None,
            who: None,
        };
        router.replace_path(router.get_path());
        router.register_callback();
        router
    }

    fn update(&mut self, page: Self::Message) {
        if let Some(who) = self.who {
            self.link.response(who, page);
        }
    }

    fn handle(&mut self, msg: Self::Input, who: HandlerId) {
        self.who = Some(who);
        match msg {
            Request::Where => {
                self.link.response(who, self.get_path());
            }
            Request::Goto(page) => {
                self.set_route(page);
            }
        }
    }
}
