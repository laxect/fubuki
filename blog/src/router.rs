use crate::Page;
use serde_derive::{Deserialize, Serialize};
use stdweb::unstable::TryFrom;
use stdweb::web::event::PopStateEvent;
use stdweb::web::window;
use stdweb::web::EventListenerHandle;
use stdweb::web::History;
use stdweb::web::IEventTarget;
use stdweb::web::Location;
use stdweb::Value;
use yew::worker::*;

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub enum Request {
    Where,
    Goto(Page),
}

impl Transferable for Request {}

pub struct Router {
    f_page: Page,
    history: History,
    location: Location,
    link: AgentLink<Router>,
    event_listener: Option<EventListenerHandle>,
    who: Option<HandlerId>,
}

impl Router {
    pub fn register_callback(&mut self) {
        let cb = self.link.send_back(|x| x);
        let fpage = self.f_page.clone();
        self.event_listener = Some(window().add_event_listener(move |event: PopStateEvent| {
            let state_value: Value = event.state();
            if let Ok(state) = String::try_from(state_value) {
                if let Ok(page) = Page::try_from(state) {
                    cb.emit(page);
                } else {
                    cb.emit(fpage.clone());
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
            .push_state(page.value(), "", Some(route.as_str()));
    }

    fn get_path(&self) -> Page {
        let mut path = self.location.pathname().unwrap();
        if path.starts_with('/') {
            path = path.replacen("/", "", 1);
        }
        Page::try_from(path).unwrap()
    }

    fn replace_path(&mut self, page: Page) {
        let mut route = page.value();
        route.insert_str(0, "/");
        let _ = self
            .history
            .replace_state(page.value(), "Gyara studio", Some(route.as_str()));
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
            f_page: Page::Index,
        };
        let f_page = router.get_path();
        router.replace_path(f_page);
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
