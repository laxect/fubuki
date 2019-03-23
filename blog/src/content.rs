use crate::markdown::render_markdown;
use crate::utils::Page;
use failure::Error;
use serde_derive::Deserialize;
use yew::format::Nothing;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{html, Callback, Component, ComponentLink, Html, Renderable, ShouldRender};

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
    pub fn inner(&self) -> String {
        match self.content {
            None => String::from(self.page.value()),
            Some(ref s) => s.clone(),
        }
    }

    pub fn load(&mut self) {
        let url = self.page.url();
        let cb = self.callback.clone();
        let handle = move |res: Response<Result<String, Error>>| {
            let (meta, body) = res.into_parts();
            if meta.status.is_success() {
                if let Ok(payload) = body {
                    cb.emit(Pong { payload });
                } else {
                    cb.emit(Pong {
                        payload: String::from("e1"),
                    })
                }
            } else {
                cb.emit(Pong {
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
        self.content = Some(msg.payload);
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props.page != self.page {
            self.page = props.page;
            self.load();
            true
        } else {
            false
        }
    }
}

impl Renderable<Content> for Content {
    fn view(&self) -> Html<Self> {
        match self.page {
            Page::Posts => {
                html! {
                    <p>{ "under construction" }</p>
                }
            }
            _ => {
                html! {
                    <>
                        <article>{ render_markdown(self.inner().as_str()) }</article>
                    </>
                }
            }
        }
    }
}
