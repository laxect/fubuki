
use crate::utils::Page;
use crate::posts::PostList;
use crate::markdown::render_markdown;
use failure::Error;
use yew::format::Nothing;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{html, Callback, Component, ComponentLink, Html, Renderable, ShouldRender};

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

pub struct Content {
    page: Page,
    web: FetchService,
    tasks: Vec<FetchTask>,
    content: Option<String>,
    post_list: Option<PostList>,
    page_callback: Callback<String>,
    post_list_callback: Callback<PostList>,
    on_change: Option<Callback<Page>>,
}

impl Content {
    pub fn inner(&self) -> String {
        match self.content {
            None => String::from(self.page.value()),
            Some(ref s) => s.clone(),
        }
    }

    pub fn page_load(&mut self) {
        let url = self.page.url();
        let cb = self.page_callback.clone();
        let handle = move |res: Response<Result<String, Error>>| {
            let (meta, body) = res.into_parts();
            if meta.status.is_success() {
                if let Ok(payload) = body {
                    cb.emit(payload);
                }
            }
        };
        let req = Request::get(url).body(Nothing).unwrap();
        let task = self.web.fetch(req, handle.into());
        self.tasks.push(task);
    }

    pub fn post_list_load(&mut self) {
        let url = String::from("/post.json");
        let cb = self.post_list_callback.clone();
        let handle = move |res: Response<Result<String, Error>>| {
            let (meta, body) = res.into_parts();
            if meta.status.is_success() {
                if let Ok(payload) = body {
                    let list: PostList = serde_json::from_str(payload.as_str()).unwrap();
                    cb.emit(list);
                }
            }
        };
        let req = Request::get(url).body(Nothing).unwrap();
        let task = self.web.fetch(req, handle.into());
        self.tasks.push(task);
    }
}

impl Component for Content {
    type Message = Msg;
    type Properties = ContentStatus;

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let mut content = Content {
            page: props.page,
            web: FetchService::new(),
            tasks: vec![],
            content: None,
            post_list: None,
            page_callback: link.send_back(Msg::Pong),
            post_list_callback: link.send_back(Msg::Posts),
            on_change: props.on_change,
        };
        content.page_load();
        content.post_list_load();
        content
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
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
            self.page = props.page;
            if self.page != Page::Posts {
                self.page_load();
            } else {
                self.content = None;
            }
            true
        } else {
            false
        }
    }
}

impl Renderable<Content> for Content {
    fn view(&self) -> Html<Self> {
        js! { @(no_return)
            let e = new CustomEvent("loaded");
            window.dispatchEvent(e);
        };
        let post_list = match &self.post_list {
            Some(list) => list.posts.clone(),
            None => vec![]
        };
        match self.page {
            Page::Posts => {
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
