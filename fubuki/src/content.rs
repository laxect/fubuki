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

impl Msg {
    pub fn into_pong(self) -> Option<String> {
        match self {
            Self::Pong(c) => Some(c),
            _ => None,
        }
    }
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
    title: Option<String>,
    front_matter: Option<FrontMatter>,
    fetch: Box<dyn Bridge<FetchAgent>>,
    on_change: Callback<Page>,
}

impl Content {
    fn inner(&self) -> Option<String> {
        self.inner
            .as_ref()
            .map(|load| match load {
                Load::Page(c) => Some(c.clone()),
                _ => None,
            })
            .flatten()
    }

    fn render_spoiler(&self) -> Html {
        match self.front_matter {
            Some(FrontMatter { ref spoiler, .. }) => spoiler.render(),
            _ => html! { <></> },
        }
    }

    fn render_title(&self) -> Html {
        match self.title {
            Some(ref title) => html! {
                <h1>{ title.clone() }</h1>
            },
            None => html! { <></> },
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
            title: None,
            front_matter: None,
            fetch: fetch_agent,
            on_change: props.on_change,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        if let Msg::Pong(ref article) = msg {
            let main: String;
            if article.starts_with("---\n") {
                if let Some(fm_end) = article[4..].find("---\n") {
                    let fm = &article[4..fm_end + 4];
                    match serde_yaml::from_str(fm) {
                        Ok(fm) => {
                            log::info!("get front_matter\n{:?}", &fm);
                            self.front_matter = Some(fm);
                        }
                        Err(e) => {
                            log::error!("fm parser failed: {}", e);
                        }
                    }
                    main = article[fm_end + 8..].to_string();
                } else {
                    unreachable!();
                }
            } else {
                main = msg.into_pong().unwrap();
            }
            if main.starts_with("# ") {
                let title_end = main.find('\n').unwrap_or_default();
                let title = &main[..title_end].replace("# ", "");
                let main = &main[title_end..];
                self.title = Some(title.to_owned());
                self.inner = Some(Load::Page(main.to_owned()));
            } else {
                self.inner = Some(Load::Page(main));
            }
        } else {
            self.inner = Some(msg.into());
        }

        // parser front_matter
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props.page != self.page {
            self.inner = None;
            self.title = None;
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
                let class = match self.page {
                    Page::Article(_) => "post",
                    _ => "",
                };
                let c = self.inner().unwrap_or_default();
                html! {
                    <main class=class>
                        { self.render_title() }
                        { self.render_spoiler() }
                        <article>{ render_markdown(&c) }</article>
                    </main>
                }
            }
        }
    }
}
