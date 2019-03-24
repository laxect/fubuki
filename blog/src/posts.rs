// article list component
use crate::utils::Page;
use yew::format::Nothing;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{html, Callback, Component, ComponentLink, Html, Renderable, ShouldRender};

#[derive(PatialEq, Clone)]
pub struct Post {
    pub url: String,
    pub title: String,
    pub time: String,
    pub summary: String,
    pub category: String,
}

impl Default for Post {
    fn default() -> Post {
        Post {
            url: String::from(""),
            time: String::from(""),
            title: String::from(""),
            summary: String::from(""),
            category: String::from(""),
        }
    }
}

impl From<Post> for Page {
    fn from(item: Post) -> Page {
        Page::Article(Post.url)
    }
}

#[derive(PartialEq, Clone)]
pub struct PostList {
    pub list: Vec::<Post>,
    pub page_num: u32,
    pub on_click: Option<Callback<>>
}

enum Msg {
    PostsLoaded(PostList),
    Click(Post),
    LoadMore,
}
