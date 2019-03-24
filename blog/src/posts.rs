// article list component
use crate::utils::Page;
use serde_derive::Deserialize;
use yew::format::Nothing;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{html, Callback, Component, ComponentLink, Html, Renderable, ShouldRender};

#[derive(PartialEq, Clone, Deserialize)]
pub struct Post {
    pub url: String,
    pub title: String,
    pub time: String,
    pub summary: String,
    pub category: String,
}

#[derive(PartialEq, Clone, Deserialize)]
pub struct PostList {
    pub posts: Vec<Post>,
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
        Page::Article(item.url)
    }
}

#[derive(Clone, PartialEq)]
pub enum Msg {
    LoadMore,
    Click(Post),
    PostsLoaded(PostList),
}

#[derive(Clone, PartialEq)]
pub struct PostsStatus {
    on_click: Option<Callback<Page>>
}

impl Default for PostsStatus {
    fn default() -> PostsStatus {
        PostsStatus {
            on_click: None,
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct Posts {
    pub page_num: u32,
    pub list: Vec<Post>,
    pub on_click: Option<Callback<Page>>
}

impl Component for Posts {
    type Message = Msg;
    type Properties = PostsStatus;

    fn create(prop: Self::Properties, _: ComponentLink<Self>) -> Self {
        Posts {
            page_num: 0,
            list: Vec::new(),
            on_click: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }
}

impl Renderable<Posts> for Posts {
    fn view(&self) -> Html<Self> {
        html! {
            <p>{ "do something" }</p>
        }
    }
}
