// article list component
use crate::utils::Page;
use serde_derive::{Deserialize, Serialize};
use yew::{html, Callback, Component, ComponentLink, Html, Renderable, ShouldRender};

#[derive(PartialEq, Clone, Deserialize, Serialize)]
pub struct Post {
    pub url: String,
    pub time: String,
    pub title: String,
    pub summary: String,
    pub category: String,
}

#[derive(PartialEq, Clone, Deserialize, Serialize)]
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
    Click(String),
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

impl From<PostList> for Msg {
    fn from(item: PostList) -> Msg {
        Msg::PostsLoaded(item)
    }
}

#[derive(Clone, PartialEq)]
pub struct PostsStatus {
    pub on_click: Option<Callback<Page>>,
    pub post_list: Vec<Post>,
}

impl Default for PostsStatus {
    fn default() -> PostsStatus {
        PostsStatus {
            on_click: None,
            post_list: Vec::new(),
        }
    }
}

pub struct Posts {
    page_num: u32,
    page_count: u32,
    list: Vec<Post>,
    on_click: Option<Callback<Page>>,
}

impl Component for Posts {
    type Message = Msg;
    type Properties = PostsStatus;

    fn create(prop: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Posts {
            page_num: 0,
            page_count: 0,
            list: prop.post_list,
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
                if let Some(ref mut on_click) = self.on_click {
                    on_click.emit(Page::Article(post));
                }
                false
            }
            Msg::PostsLoaded(post_list) => {
                self.list = post_list.posts;
                self.page_count = (self.list.len() as u32 + 4) / 5;
                if self.page_count > 0 {
                    self.page_count -= 1;
                }
                true
            }
        }
    }

    fn change(&mut self, prop: Self::Properties) -> ShouldRender {
        // never change
        self.list = prop.post_list;
        true
    }
}

impl Renderable<Posts> for Posts {
    fn view(&self) -> Html<Self> {
        // article item
        let article = |ind| -> Html<Self> {
            let post = &self.list[ind as usize];
            let url = post.url.clone();
            html! {
                <article>
                    <h3><button onclick=|_| Msg::Click(url.clone()), >{ &post.title }</button></h3>
                    <p>{ &post.summary }</p>
                    <small><time>{ &post.time }</time><span class="category", >{ &post.category }</span></small>
                </article>
            }
        };
        // pagnation link item
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
                <button class=class,
                    onclick=|_| item.clone(), >
                    { mark }
                </button>
            }
        };
        let start = self.page_num * 5;
        let end = std::cmp::min(start + 5, self.list.len() as u32);
        html! {
            <main>
                { for (start..end).map(article) }
                <nav class="nav post-nav", >
                    { link(Msg::Prev) }
                    { link(Msg::Next) }
                </nav>
            </main>
        }
    }
}
