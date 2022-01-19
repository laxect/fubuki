use crate::{
    fetch_agent::{FetchAgent, Load},
    markdown::render_markdown,
    utils::Page,
};
use fubuki_types::{FrontMatter, Spoiler};
use yew::{
    html, Bridge, Bridged, Callback, Component, ComponentLink, Html, Properties, ShouldRender,
};

#[derive(PartialEq, Clone, Properties)]
pub struct ContentStatus {
    pub page: Page,
    pub on_change: Callback<Page>,
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
            Some(FrontMatter { ref spoiler, .. }) => match spoiler {
                Spoiler::None => html! {
                    <></>
                },
                Spoiler::Some { target, level } => {
                    html! {
                        <p class="spoiler-alert">
                        { format!("请注意，本文可能含有对{}的 ", target) }
                        <span class="spoiler-level">{ level }</span>
                        { " 等级剧透。" }
                        </p>
                    }
                }
            },
            _ => html! { <></> },
        }
    }

    fn render_title(&self) -> Html {
        let fm_title = self.front_matter.as_ref().map(|f| &f.title);
        let title = self.title.as_ref().or(fm_title);
        match title {
            Some(title) => html! {
                <h1>{ &title.clone() }</h1>
            },
            None => html! { <></> },
        }
    }
}

impl Component for Content {
    type Message = Load;
    type Properties = ContentStatus;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|x| x);
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
        if let Load::Page(ref article) = msg {
            let main: String;
            // remove front matter
            if article.starts_with("---\n") {
                if let Some(fm) = article.split("---\n").nth(1) {
                    // ---\n..---\n
                    match serde_yaml::from_str(fm) {
                        Ok(fm) => {
                            log::info!("get front_matter\n{:?}", &fm);
                            self.front_matter = Some(fm);
                        }
                        Err(e) => {
                            log::error!("fm parser failed: {}", e);
                        }
                    }
                    main = article[fm.len() + 8..].to_string();
                } else {
                    unreachable!();
                }
            } else {
                main = msg.into_page().unwrap();
            }
            // checkout title
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
            self.inner = Some(msg);
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
                        <crate::posts::Posts on_click=self.on_change.clone() post_list=post_list/>
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
