use crate::Page;
use serde_derive::{Deserialize, Serialize};
use std::convert::TryFrom;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{window, History, Location, PopStateEvent};
use yew::worker::*;

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub enum Request {
    Where,
    Goto(Page),
    Reload(bool),
}

fn set_title(title: &str) {
    window().unwrap().document().unwrap().set_title(title);
}

pub struct Router {
    history: History,
    location: Location,
    link: AgentLink<Router>,
    who: Option<HandlerId>,
}

impl Router {
    pub fn register_callback(&mut self) {
        let cb = self.link.callback(|x| x);
        let handle = Closure::wrap(Box::new(move |event: PopStateEvent| {
            let state_value: JsValue = event.state();
            if let Some(state) = state_value.as_string() {
                if let Ok(page) = Page::try_from(state) {
                    cb.emit(page);
                }
            } else {
                eprintln!("Nothing farther back in history, not calling routing callback.");
            }
        }) as Box<dyn FnMut(PopStateEvent)>);
        let _ = window()
            .unwrap()
            .add_event_listener_with_callback("popstate", handle.as_ref().unchecked_ref());
    }

    fn set_route(&mut self, page: Page) {
        let mut route = page.value();
        route.insert_str(0, "/");
        let _ = self.history.push_state_with_url(
            &(page.value().into()),
            &page.title(),
            Some(route.as_str()),
        );
        set_title(&page.title());
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
        let _ = self.history.replace_state_with_url(
            &(page.value().into()),
            &page.title(),
            Some(route.as_str()),
        );
        set_title(&page.title());
    }

    fn reload(&self, forced_reload: bool) {
        let _ = self.location.reload_with_forceget(forced_reload);
    }
}

impl Agent for Router {
    type Reach = Context;
    type Message = Page;
    type Input = Request;
    type Output = Page;

    fn create(link: AgentLink<Self>) -> Router {
        let window = window().unwrap();
        let location = window.location();
        let mut router = Router {
            link,
            history: window.history().unwrap(),
            location,
            who: None,
        };
        router.replace_path(router.get_path());
        router.register_callback();
        router
    }

    fn update(&mut self, page: Self::Message) {
        if let Some(who) = self.who {
            self.link.respond(who, page);
        }
    }

    fn handle_input(&mut self, msg: Self::Input, who: HandlerId) {
        self.who = Some(who);
        match msg {
            Request::Where => {
                self.link.respond(who, self.get_path());
            }
            Request::Goto(page) => {
                self.set_route(page);
            }
            Request::Reload(forced_reload) => {
                self.reload(forced_reload);
            }
        }
    }
}
