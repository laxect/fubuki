use yew::services::FetchService;
use yew::{html, Callback, Component, ComponentLink, Html, Renderable, ShouldRender};
use crate::utils::Page;

pub struct ContentStatus {
    page: Page,
    on_change: Option<Callback<Page>>,
}

impl Default for ContentStatus {
    fn default() -> ContentStatus {
        ContentStatus {
            page: Page::Index,
            on_change: None,
        }
    }
}

pub struct Content {
    page: Page,
    web: FetchService,
}

impl Content {
    pub fn new(page: Page) -> Content {
        Content {
            page,
            web: FetchService::new(),
        }
    }
}

