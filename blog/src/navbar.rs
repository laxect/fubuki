use crate::utils::Page;
use yew::{html, Callback, Component, ComponentLink, Html, Renderable, ShouldRender};

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
            on_change: prop.on_change,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        if msg != self.page {
            self.page = msg.clone();
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
            let mark = item.value();
            if item == self.page {
                html! {
                    <a class="nav-link active",
                        onclick=|_| item.clone(), >
                        { mark }
                    </a>
                }
            } else {
                html! {
                    <a class="nav-link",
                        onclick=|_| item.clone(), >
                        { mark }
                    </a>
                }
            }
        };
        // nav bar
        html! {
            <nav class="nav", >
                { link(Page::Index) }
                { link(Page::Friend) }
                { link(Page::Posts) }
                { link(Page::About) }
            </nav>
        }
    }
}
