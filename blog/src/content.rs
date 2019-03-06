use std::boxed::Box;
use failure::Error;
use serde_derive::Deserialize;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{html, Callback, Component, ComponentLink, Html, Renderable, ShouldRender};
use crate::utils::Page;

#[derive(PartialEq, Clone)]
pub struct ContentStatus {
    pub page: Page,
    pub on_change: Option<Callback<Page>>,
}

impl Default for ContentStatus {
    fn default() -> ContentStatus {
        ContentStatus {
            page: Page::Index,
            on_change: None,
        }
    }
}

// msg
#[derive(PartialEq, Clone, Deserialize)]
pub struct Pong {
    status: String,
    payload: String,
}

pub struct Content {
    page: Page,
    web: FetchService,
    task: Option<FetchTask>,
    content: Option<String>,
    callback: Callback<Pong>,
}

impl Content {
    pub fn inner(&self) -> &'static str {
        match self.content {
            None => self.page.value(),
            Some(ref s) => Box::leak(s.clone().into_boxed_str())
        }
    }

    pub fn load(&mut self) {
        let url = String::from("http://127.0.0.1:3002/ping");
        let cb = self.callback.clone();
        let handle = move |res: Response<Json<Result<Pong, Error>>>| {
            let (meta, Json(data)) = res.into_parts();
            if meta.status.is_success() {
                if let Ok(pong) = data {
                    cb.emit(pong);
                } else {
                    cb.emit(Pong {
                        status: String::from("OK"),
                        payload: String::from("e1"),
                    });
                }
            } else {
                cb.emit(Pong {
                    status: String::from("OK"),
                    payload: String::from("e2"),
                });
            }
        };
        let req = Request::get(url).body(Nothing).unwrap();
        let task = self.web.fetch(req, handle.into());
        self.task = Some(task);
    }
}

impl Component for Content {
    type Message = Pong;
    type Properties = ContentStatus;

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let mut content = Content {
            page: props.page,
            web: FetchService::new(),
            content: None,
            task: None,
            callback: link.send_back(|pong| pong),
        };
        content.load();
        content
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        if msg.status == String::from("OK") {
            self.content = Some(msg.payload);
            true
        } else {
            false
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props.page != self.page {
            self.page = props.page;
            true
        } else {
            false
        }
    }
}

impl Renderable<Content> for Content {
    fn view(&self) -> Html<Self> {
        html! {
            <>
            <p>{ self.inner() }</p>
            </>
        }
    }
}
