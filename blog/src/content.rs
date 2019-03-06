use yew::services::fetch::{ FetchService, Request, Response };
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
#[derive(PartialEq, Clone, Copy)]
pub struct Pong {
    status: bool,
    content: &'static str,
}

pub struct Content {
    page: Page,
    web: FetchService,
    content: Option<&'static str>
}

impl Content {
    pub fn new(page: Page) -> Content {
        Content {
            page,
            web: FetchService::new(),
            content: None,
        }
    }

    pub fn load(page: Page) {

    }
}

impl Component for Content {
    type Message = Pong;
    type Properties = ContentStatus;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Content::new(props.page)
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        if msg.status == true {
            self.content = Some(msg.content);
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
            <p>{ self.page.value() }</p>
            </>
        }
    }
}
