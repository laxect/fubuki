use crate::cache::{Cache, CacheContent};
use crate::fetch_agent::{FetchAgent, Load};
use crate::markdown::render_markdown;
use crate::posts::PostList;
use crate::utils::Page;
use yew::*;

#[derive(PartialEq, Clone)]
pub struct ContentStatus {
    pub page: Page,
    pub on_change: Option<Callback<Page>>,
}

impl Default for ContentStatus {
    fn default() -> ContentStatus {
        ContentStatus {
            page: Page::Index,
            on_change: None,
        }
    }
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
            Load::PostList(postlist) => Msg::Posts(postlist),
        }
    }
}

pub struct Content {
    page: Page,
    cache: Cache,
    fetch: Box<Bridge<FetchAgent>>,
    on_change: Option<Callback<Page>>,
}

impl Content {
    fn inner(&self) -> Option<String> {
        if let Some(CacheContent::Page(ref c)) = self.cache.get(&self.page) {
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

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(Msg::from);
        let mut fetch_agent = FetchAgent::bridge(callback);
        fetch_agent.send(props.page.clone());
        Content {
            page: props.page,
            cache: Cache::new(),
            fetch: fetch_agent,
            on_change: props.on_change,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.cache.set(self.page.clone(), msg.clone().into());
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props.page != self.page {
            self.page = props.page.clone();
            if !self.cache.has(&props.page) {
                self.fetch.send(props.page);
            }
            true
        } else {
            false
        }
    }
}

impl Renderable<Content> for Content {
    fn view(&self) -> Html<Self> {
        if !self.cache.has(&self.page) {
            html! {
                <main>
                    <div class="bubblingG", >
                        <span id="bubblingG_0", ></span>
                        <span id="bubblingG_1", ></span>
                        <span id="bubblingG_2", ></span>
                    </div>
                </main>
            }
        } else {
            match self.page {
                Page::Posts => {
                    let post_list = match self.cache.get(&self.page) {
                        Some(CacheContent::Posts(list)) => list,
                        _ => vec![],
                    };
                    html! {
                        <crate::posts::Posts: on_click=self.on_change.clone(), post_list=post_list, />
                    }
                }
                _ => {
                    let c = self.inner().unwrap();
                    let class = match self.page {
                        Page::Article(_) => "post",
                        _ => "",
                    };
                    html! {
                        <main class=class, >
                            <article>{ render_markdown(&c) }</article>
                        </main>
                    }
                }
            }
        }
    }
}
