// article list component
use yew::format::Nothing;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{html, Callback, Component, ComponentLink, Html, Renderable, ShouldRender};

#[derive(PatialEq, Clone)]
pub struct Artcile {
    pub title: String,
    pub time; String,
    pub summary: String,
    pub category: String,
}

#[derive(PartialEq, Clone)]
pub struct ArtcileList {
    pub list: Vec::<Artcile>,
    pub page_num: u32,
    pub on_click: Option<Callback<>>
}

impl Default for ArtcileList {
    fn default() -> ArtcileList {
        ArtcileList {
            list: Vec::new(),
            page_num: 0,
            on_click: None,
        }
    }
}
