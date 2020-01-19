use crate::{
    fetch_agent::{FetchAgent, Load},
    markdown::render_markdown,
    posts::PostList,
    utils::Page,
};
use yew::*;

#[derive(PartialEq, Clone, Properties)]
pub struct ContentStatus {
    #[props(required)]
    pub page: Page,
    #[props(required)]
    pub on_change: Callback<Page>,
}

#[derive(PartialEq, Clone)]
pub enum Msg {
    Pong(String),
    Posts(PostList),
}

impl From<Load> for Msg {
    fn from(load: Load) -> Msg {
        match load {
            Load::Page(payload) => Msg::Pong(payload),
            Load::Posts(postlist) => Msg::Posts(postlist),
        }
    }
}

pub struct Content {
    page: Page,
    inner: Option<Load>,
    fetch: Box<dyn Bridge<FetchAgent>>,
    on_change: Callback<Page>,
}

impl Content {
    fn inner(&self) -> Option<String> {
        if let Some(Load::Page(ref c)) = self.inner {
            if c.starts_with("---\n") {
                let after = &c[4..];
                if let Some(ind) = after.find("---\n") {
                    return Some(c[ind + 4 * 2..].into());
                }
            }
            Some(c.clone())
        } else {
            None
        }
    }
}

impl Component for Content {
    type Message = Msg;
    type Properties = ContentStatus;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(Msg::from);
        let mut fetch_agent = FetchAgent::bridge(callback);
        fetch_agent.send(props.page.clone());
        Content {
            page: props.page,
            inner: None,
            fetch: fetch_agent,
            on_change: props.on_change,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.inner = Some(msg.into());
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props.page != self.page {
            self.inner = None;
            self.page = props.page.clone();
            self.fetch.send(props.page);
        }
        false
    }

    fn view(&self) -> Html {
        if self.inner.is_none() {
            return html! {
                <main>
                </main>
            };
        }
        match self.page {
            Page::Posts => {
                let post_list = match self.inner.clone() {
                    Some(Load::Posts(post_list)) => post_list,
                    _ => Vec::new(),
                };
                html! {
                    <main class="post-list">
                        <crate::posts::Posts: on_click=self.on_change.clone() post_list=post_list/>
                    </main>
                }
            }
            _ => {
                let c = self.inner().unwrap();
                let class = match self.page {
                    Page::Article(_) => "post",
                    _ => "",
                };
                html! {
                    <main class=class>
                        <article>{ render_markdown(&c) }</article>
                    </main>
                }
            }
        }
    }
}
