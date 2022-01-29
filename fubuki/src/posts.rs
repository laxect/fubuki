// article list component
use crate::utils::Page;
pub use fubuki_types::{Post, PostList};
use yew::{html, Callback, Component, Context, Html, Properties};

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
fn page_count(ctx: &Context<Posts>) -> u32 {
    (ctx.props().post_list.len() as u32 + 4) / 5
}

#[derive(Clone, PartialEq, Properties)]
pub struct PostsStatus {
    pub on_click: Callback<Page>,
    #[prop_or_default]
    pub post_list: Vec<Post>,
}

pub struct Posts {
    page_num: u32,
}

impl Posts {
    fn page_flip(&mut self, msg: &Msg, page_count: u32) -> bool {
        let mut new_page_num = match msg {
            Msg::Prev => self.page_num.saturating_sub(1),
            Msg::Next => self.page_num.saturating_add(1),
            // Msg::Click
            _ => unreachable!(),
        };
        if new_page_num > page_count {
            new_page_num = page_count;
        }
        let res = self.page_num != new_page_num;
        self.page_num = new_page_num;
        res
    }

    fn article_list_html(&self, ctx: &Context<Self>) -> Html {
        let article_html = |post: &Post| -> Html {
            let url = post.url.clone();
            let onclick = ctx.link().callback(move |_| Msg::Click(url.clone()));
            html! {
                <article>
                    <h2 class="post-title">
                        <button class="post-title" {onclick}>{ &post.title }</button>
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
        let end = std::cmp::min(start + 5, ctx.props().post_list.len());
        html! {
            <>
            { for ctx.props().post_list.get(start..end).unwrap_or(&[]).iter().map(article_html) }
            </>
        }
    }
}

impl Component for Posts {
    type Message = Msg;
    type Properties = PostsStatus;

    fn create(_ctx: &Context<Self>) -> Self {
        Posts { page_num: 0 }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Click(post) => {
                ctx.props().on_click.emit(Page::Article(post));
                false
            }
            _ => self.page_flip(&msg, page_count(ctx)),
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        // never change
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // pagnation link item
        let page_count = page_count(ctx);
        let link = |item: Msg| -> Html {
            let disabled = match item {
                Msg::Next => self.page_num + 1 >= page_count,
                Msg::Prev => self.page_num == 0,
                _ => unreachable!(),
            };
            let mark = item.value();
            let class = if disabled {
                "btn btn-post disable"
            } else {
                "btn btn-post"
            };
            let onclick = ctx.link().callback(move |_| item.clone());
            html! {
                <button {class} {onclick}>
                    { mark }
                </button>
            }
        };
        html! {
            <>
                { self.article_list_html(ctx) }
                <nav class="post-nav" style="float: right">
                    { link(Msg::Prev) }
                    { link(Msg::Next) }
                </nav>
            </>
        }
    }
}
