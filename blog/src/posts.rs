// article list component
use crate::utils::Page;
use serde_derive::{Deserialize, Serialize};
use yew::{html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(PartialEq, Clone, Deserialize, Serialize)]
pub struct Post {
    pub url: String,
    pub date: String,
    pub title: String,
    pub summary: String,
    pub category: String,
    tags: Vec<String>,
}

pub type PostList = Vec<Post>;

impl Default for Post {
    fn default() -> Post {
        Post {
            url: String::from(""),
            date: String::from(""),
            title: String::from(""),
            summary: String::from(""),
            category: String::from(""),
            tags: Vec::new(),
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
    Click(String),
}

impl Msg {
    pub fn value(&self) -> &'static str {
        match self {
            Msg::Prev => "← Prev",
            Msg::Next => "Next →",
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct PostsStatus {
    #[props(required)]
    pub on_click: Callback<Page>,
    pub post_list: Vec<Post>,
}

pub struct Posts {
    page_num: u32,
    page_count: u32,
    list: Vec<Post>,
    on_click: Callback<Page>,
    link: ComponentLink<Self>,
}

impl Component for Posts {
    type Message = Msg;
    type Properties = PostsStatus;

    fn create(prop: Self::Properties, link: ComponentLink<Self>) -> Self {
        Posts {
            page_num: 0,
            page_count: (prop.post_list.len() as u32 + 4) / 5,
            list: prop.post_list,
            on_click: prop.on_click,
            link,
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
                self.on_click.emit(Page::Article(post));
                false
            }
        }
    }

    fn change(&mut self, prop: Self::Properties) -> ShouldRender {
        // never change
        self.page_count = (prop.post_list.len() as u32 + 4) / 5;
        self.list = prop.post_list;
        true
    }

    fn view(&self) -> Html {
        // article item
        let article = |ind| -> Html {
            let post = &self.list[ind as usize];
            let url = post.url.clone();
            let on_click_top = self.link.callback(move |_| Msg::Click(url.clone()));
            html! {
                <article>
                    <h2 class="post-title">
                        <button class="post-title" onclick=on_click_top>{ &post.title }</button>
                    </h2>
                    <p>{ &post.summary }</p>
                    <small><time>{ &post.date }</time><span class="category">{ &post.category }</span></small>
                </article>
            }
        };
        // pagnation link item
        let link = |item: Msg| -> Html {
            let disabled = match item {
                Msg::Next => self.page_num + 1 >= self.page_count,
                Msg::Prev => self.page_num == 0,
                _ => unreachable!(),
            };
            let mark = item.value();
            let class = if disabled {
                "btn btn-post disable"
            } else {
                "btn btn-post"
            };
            let on_click = self.link.callback(move |_| item.clone());
            html! {
                <button class=class
                    onclick=on_click>
                    { mark }
                </button>
            }
        };
        let start = self.page_num * 5;
        let end = std::cmp::min(start + 5, self.list.len() as u32);
        html! {
            <>
                { for (start..end).map(article) }
                <nav class="post-nav">
                    { link(Msg::Prev) }
                    { link(Msg::Next) }
                </nav>
            </>
        }
    }
}
