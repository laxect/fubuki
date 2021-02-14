// article list component
use crate::utils::Page;
pub use fubuki_types::{Post, PostList};
use yew::{html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};

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

#[inline]
fn page_count(post_num: usize) -> u32 {
    (post_num as u32 + 4) / 5
}

#[derive(Clone, PartialEq, Properties)]
pub struct PostsStatus {
    pub on_click: Callback<Page>,
    #[prop_or_default]
    pub post_list: Vec<Post>,
}

pub struct Posts {
    page_num: u32,
    page_count: u32,
    list: Vec<Post>,
    on_click: Callback<Page>,
    link: ComponentLink<Self>,
}

impl Posts {
    fn page_flip(&mut self, msg: &Msg) -> bool {
        let mut new_page_num = match msg {
            Msg::Prev => self.page_num.saturating_sub(1),
            Msg::Next => self.page_num.saturating_add(1),
            // Msg::Click
            _ => unreachable!(),
        };
        if new_page_num > self.page_count {
            new_page_num = self.page_count;
        }
        let res = self.page_num != new_page_num;
        self.page_num = new_page_num;
        res
    }

    fn article_list_html(&self) -> Html {
        let article_html = |post: &Post| -> Html {
            let url = post.url.clone();
            let on_click = self.link.callback(move |_| Msg::Click(url.clone()));
            html! {
                <article>
                    <h2 class="post-title">
                        <button class="post-title" onclick=on_click>{ &post.title }</button>
                    </h2>
                    <p>{ &post.summary }</p>
                    <small>
                        <time>{ &post.date }</time><span class="category">{ &post.category }</span>
                        {
                            if post.has_spoiler() {
                                html! {
                                    <span class="spoiler">{ "ネタバレ注意" }</span>
                                }
                            } else {
                                html! {}
                            }
                        }
                    </small>
                </article>
            }
        };
        let start = self.page_num as usize * 5;
        let end = std::cmp::min(start + 5, self.list.len());
        html! {
            <>
            { for self.list.get(start..end).unwrap_or(&[]).into_iter().map(article_html) }
            </>
        }
    }
}

impl Component for Posts {
    type Message = Msg;
    type Properties = PostsStatus;

    fn create(prop: Self::Properties, link: ComponentLink<Self>) -> Self {
        Posts {
            page_num: 0,
            page_count: page_count(prop.post_list.len()),
            list: prop.post_list,
            on_click: prop.on_click,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click(post) => {
                self.on_click.emit(Page::Article(post));
                false
            }
            _ => self.page_flip(&msg),
        }
    }

    fn change(&mut self, prop: Self::Properties) -> ShouldRender {
        // never change
        self.page_count = page_count(prop.post_list.len());
        self.list = prop.post_list;
        true
    }

    fn view(&self) -> Html {
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
        html! {
            <>
                { self.article_list_html() }
                <nav class="post-nav" style="float: right">
                    { link(Msg::Prev) }
                    { link(Msg::Next) }
                </nav>
            </>
        }
    }
}
