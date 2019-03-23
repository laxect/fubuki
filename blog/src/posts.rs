// article list component
use yew::format::Nothing;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{html, Callback, Component, ComponentLink, Html, Renderable, ShouldRender};

#[derive(PatialEq, Clone)]
pub struct Post {
    pub title: String,
    pub time; String,
    pub summary: String,
    pub category: String,
}

#[derive(PartialEq, Clone)]
pub struct PostList {
    pub list: Vec::<Post>,
    pub page_num: u32,
    pub on_click: Option<Callback<>>
}
