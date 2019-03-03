use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct NavBar {
    status: Page,
}

#[derive(PartialEq, Clone)]
pub enum Page {
    Index,
    Article,
    About,
}

impl Page {
    fn value(&self) -> &'static str {
        match self {
            Page::Index => "index",
            Page::Article => "post",
            Page::About => "about",
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct NavStatus {
    pub page: Page,
}

impl Default for NavStatus {
    fn default() -> NavStatus {
        NavStatus { page: Page::Index }
    }
}

impl Component for NavBar {
    type Message = Page;
    type Properties = NavStatus;

    fn create(prop: Self::Properties, _: ComponentLink<Self>) -> Self {
        NavBar { status: prop.page }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        if msg != self.status {
            return true;
        }
        true
    }
}

impl Renderable<NavBar> for NavBar {
    fn view(&self) -> Html<Self> {
        let link = |item: Page| -> Html<Self> {
            html! {
                <a class="nav-link", >
                    { item.value() }
                </a>
            }
        };
        html! {
            <nav class="nav", >
                { link(Page::Index) }
                { link(Page::Article) }
                { link(Page::About) }
            </nav>
        }
    }
}
