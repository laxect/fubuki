use crate::Page;
use serde_derive::{Deserialize, Serialize};
use std::convert::TryFrom;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{window, History, Location, PopStateEvent};
use yew_agent::{Agent, AgentLink, Context, HandlerId};

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub enum Request {
    Where,
    Goto(Page),
    Reload(bool),
}

fn set_title(title: &str) -> Result<(), &'static str> {
    window()
        .ok_or("open window failed")?
        .document()
        .ok_or("document open failed")?
        .set_title(title);
    Ok(())
}

pub struct Router {
    history: History,
    location: Location,
    link: AgentLink<Router>,
    who: Option<HandlerId>,
    _pop_state_handle: Option<Closure<dyn FnMut(PopStateEvent)>>,
}

impl Router {
    fn register_callback(&mut self) {
        let cb = self.link.callback(|x| x);
        let handle = Closure::wrap(Box::new(move |event: PopStateEvent| {
            if let Some(Ok(page)) = event.state().as_string().map(Page::try_from) {
                cb.emit(page);
            }
        }) as Box<dyn FnMut(PopStateEvent)>);
        let _ = window()
            .expect("open window and failed")
            .add_event_listener_with_callback("popstate", handle.as_ref().unchecked_ref());
        self._pop_state_handle = Some(handle);
    }

    fn set_path(&mut self, page: Page) {
        let route = format!("/{}", page.value());
        let _ = self.history.push_state_with_url(
            &(page.value().into()),
            &page.title(),
            Some(route.as_str()),
        );
        let _ = set_title(&page.title());
    }

    fn get_path(&self) -> Page {
        let mut path = self.location.pathname().unwrap_or_else(|_| "gs".to_owned());
        if path.starts_with('/') {
            path = path.replacen('/', "", 1);
        }
        if path.ends_with('/') {
            path.pop();
        }
        Page::try_from(path).unwrap()
    }

    fn replace_path(&mut self, page: Page) {
        let mut route = page.value();
        route.insert(0, '/');
        let _ = self.history.replace_state_with_url(
            &(page.value().into()),
            &page.title(),
            Some(route.as_str()),
        );
        let _ = set_title(&page.title());
    }

    fn reload(&self, forced_reload: bool) {
        let _ = self.location.reload_with_forceget(forced_reload);
    }
}

impl Agent for Router {
    type Reach = Context<Self>;
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
            _pop_state_handle: None,
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
                self.set_path(page);
            }
            Request::Reload(forced_reload) => {
                self.reload(forced_reload);
            }
        }
    }
}
