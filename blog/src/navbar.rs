use yew::{html, Callback, Component, ComponentLink, Html, Renderable, ShouldRender};

#[derive(PartialEq, Clone, Copy)]
pub enum Page {
    Index,
    Article,
    About,
    Friend,
}

impl Page {
    pub fn value(&self) -> &'static str {
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
    pub on_change: Option<Callback<Page>>,
}

impl Default for NavStatus {
    fn default() -> NavStatus {
        NavStatus {
            page: Page::Index,
            on_change: None,
        }
    }
}

pub struct NavBar {
    page: Page,
    on_change: Option<Callback<Page>>,
}

impl Component for NavBar {
    type Message = Page;
    type Properties = NavStatus;

    fn create(prop: Self::Properties, _: ComponentLink<Self>) -> Self {
        NavBar {
            page: prop.page,
            on_change:prop.on_change,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        if msg != self.page {
            self.page = msg;
            if let Some(ref mut cb) = self.on_change {
                cb.emit(msg);
            }
            true
        } else {
            false
        }
    }

    fn change(&mut self, prop: Self::Properties) -> ShouldRender {
        self.page = prop.page;
        self.on_change = prop.on_change;
        true
    }
}

impl Renderable<NavBar> for NavBar {
    fn view(&self) -> Html<Self> {
        // link item
        let link = |item: Page| -> Html<Self> {
            if item == self.page {
                html! {
                    <a class="nav-link active",
                        onclick=|_| item, >
                        { item.value() }
                    </a>
                }
            } else {
                html! {
                    <a class="nav-link",
                        onclick=|_| item, >
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
