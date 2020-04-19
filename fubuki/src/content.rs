use crate::{
    fetch_agent::{FetchAgent, Load},
    markdown::render_markdown,
    posts::PostList,
    utils::Page,
};
use serde::Deserialize;
use yew::*;

#[derive(PartialEq, Clone, Properties)]
pub struct ContentStatus {
    pub page: Page,
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

#[derive(Debug, Deserialize)]
struct FrontMatter {
    title: String,
    category: String,
    tags: Vec<String>,
    summary: String,
    date: String,
    #[serde(default)]
    spoiler: Spoiler,
}

#[derive(Debug, Deserialize)]
enum Spoiler {
    None,
    Some { target: String, level: u32 },
}

impl Default for Spoiler {
    fn default() -> Self {
        Self::None
    }
}

impl Spoiler {
    pub fn render(&self) -> Html {
        match self {
            Self::None => html! {
                <></>
            },
            Self::Some { target, level } => {
                html! {
                    <p class="spoiler-alert">
                    { format!("请注意，本文可能含有对{}的 ", target) }
                    <span class="spoiler-level">{ level }</span>
                    { " 等级剧透。" }
                    </p>
                }
            }
        }
    }
}

pub struct Content {
    page: Page,
    inner: Option<Load>,
    front_matter: Option<FrontMatter>,
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

    fn render_spoiler(&self) -> Html {
        match self.front_matter {
            Some(FrontMatter { ref spoiler, .. }) => spoiler.render(),
            _ => html! { <></> },
        }
    }
}

impl Component for Content {
    type Message = Msg;
    type Properties = ContentStatus;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(Msg::from);
        let mut fetch_agent = FetchAgent::bridge(callback);
        fetch_agent.send(props.page.clone().into());
        Content {
            page: props.page,
            inner: None,
            front_matter: None,
            fetch: fetch_agent,
            on_change: props.on_change,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.inner = Some(msg.into());
        if let Some(Load::Page(ref article)) = self.inner {
            if article.starts_with("---\n") {
                if let Some(fm_end) = article[4..].find("---\n") {
                    let fm = &article[4..fm_end];
                    if let Ok(fm) = serde_yaml::from_str(fm) {
                        log::info!("get front_matter\n{:?}", &fm);
                        self.front_matter = Some(fm);
                    }
                }
            }
        }
        // parser front_matter
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props.page != self.page {
            self.inner = None;
            self.front_matter = None;
            self.page = props.page.clone();
            self.fetch.send(props.page.into());
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
                let title_end = c.find('\n').unwrap_or_default();
                let title = &c[..title_end].replace("# ", "");
                let c = &c[title_end..];
                let class = match self.page {
                    Page::Article(_) => "post",
                    _ => "",
                };
                html! {
                    <main class=class>
                        <h1>{ title }</h1>
                        { self.render_spoiler() }
                        <article>{ render_markdown(c) }</article>
                    </main>
                }
            }
        }
    }
}
