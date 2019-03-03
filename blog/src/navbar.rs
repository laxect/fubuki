use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

#[derive(PartialEq, Clone)]
pub enum Page {
    Index,
    Article,
    About,
    Friend,
}

impl Page {
    fn value(&self) -> &'static str {
        match self {
            Page::Index => "index",
            Page::Article => "post",
            Page::About => "about",
            Page::Friend => "Friends",
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

pub struct NavBar {
    status: Page,
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
        // link item
        let link = |item: Page| -> Html<Self> {
            if item == self.status {
                html! {
                    <a class="nav-link active", >
                        { item.value() }
                    </a>
                }
            } else {
                html! {
                    <a class="nav-link", >
                        { item.value() }
                    </a>
                }
            }
        };
        // nav bar
        html! {
            <nav class="nav", >
                { link(Page::Index) }
                { link(Page::Article) }
                { link(Page::Friend) }
                { link(Page::About) }
            </nav>
        }
    }
}
