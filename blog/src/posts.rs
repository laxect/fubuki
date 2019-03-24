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
    Prev,
    Next,
    Click(Post),
    PostsLoaded(PostList),
}

impl Msg {
    pub fn value(&self) -> &'static str {
        match self {
            Msg::Prev => "Prev",
            Msg::Next => "Next",
            _ => unreachable!(),
        }
    }
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
    pub page_count: u32,
    pub list: Vec<Post>,
    pub on_click: Option<Callback<Page>>
}

impl Component for Posts {
    type Message = Msg;
    type Properties = PostsStatus;

    fn create(prop: Self::Properties, _: ComponentLink<Self>) -> Self {
        Posts {
            page_num: 0,
            page_count: 10,
            list: Vec::new(),
            on_click: prop.on_click,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Prev => {
                if self.page_num != 0 {
                    self.page_num -= 1;
                    true
                } else {
                    false
                }
            }
            Msg::Next => {
                if self.page_num != self.page_count {
                    self.page_num += 1;
                    true
                } else {
                    false
                }
            }
            Msg::Click(post) => {
                false
            }
            Msg::PostsLoaded(post_list) => {
                self.list = post_list.posts;
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        // never change
        false
    }
}

impl Renderable<Posts> for Posts {
    fn view(&self) -> Html<Self> {
        // link item
        let link = |item: Msg| -> Html<Self> {
            let disabled = match item {
                Msg::Next => self.page_num == self.page_count,
                Msg::Prev => self.page_num == 0,
                _ => unreachable!(),
            };
            let mark = item.value();
            let class = if disabled {
                "btn btn-post disable"
            } else {
                "btn btn-post"
            };
            html! {
                <a class=class,
                    onclick=|_| item.clone(), >
                    { mark }
                </a>
            }
        };
        html! {
            <>
                <p>{ self.page_num }</p>
                <nav class="nav", >
                    { link(Msg::Prev) }
                    { link(Msg::Next) }
                </nav>
            </>
        }
    }
}
