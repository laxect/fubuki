use crate::fetch_agent::{FetchAgent, Load};
use crate::markdown::render_markdown;
use crate::posts::PostList;
use crate::utils::Page;
use crate::cache::{ Cache, CacheContent };
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
    fetch: Box<Bridge<FetchAgent>>,
    content: Option<String>,
    post_list: Option<PostList>,
    on_change: Option<Callback<Page>>,
    cache: Cache,
}

impl Content {
    fn inner(&self) -> String {
        match self.content {
            None => "".into(),
            Some(ref s) => {
                if let Some(c) = self.find_content() {
                    return c;
                }
                s.clone()
            }
        }
    }

    fn find_content(&self) -> Option<String> {
        if let Some(ref c) = self.content {
            if c.starts_with("---\n") {
                let after = &c[4..];
                if let Some(ind) = after.find("---\n") {
                    return Some(c[ind+4*2..].into());
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
            fetch: fetch_agent,
            content: None,
            post_list: None,
            on_change: props.on_change,
            cache: Cache::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.cache.set(self.page.clone(), msg.clone().into());
        match msg {
            Msg::Pong(pong) => {
                self.content = Some(pong);
            }
            Msg::Posts(post_list) => {
                self.post_list = Some(post_list);
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props.page != self.page {
            self.page = props.page.clone();
            if let Some(cache) = self.cache.get(&props.page) {
                match cache {
                    CacheContent::Page(content) => {self.content = Some(content);}
                    CacheContent::Posts(pl) => {self.post_list = Some(pl);}
                }
            } else {
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
        match self.page {
            Page::Posts => {
                let post_list = match &self.post_list {
                    Some(list) => list.posts.clone(),
                    None => vec![],
                };
                html! {
                    <crate::posts::Posts: on_click=self.on_change.clone(), post_list=post_list, />
                }
            }
            _ => {
                html! {
                    <main>
                        <article>{ render_markdown(self.inner().as_str()) }</article>
                    </main>
                }
            }
        }
    }
}
