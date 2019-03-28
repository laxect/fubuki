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
            let mut post = false;
            if let Page::Article(_) = self.page {
                post = true;
            };
            if post && item == Page::Posts {
                html! {
                    <button class="nav-link",
                        onclick=|_| item.clone(), >
                        <span class="mark", >{ "post" }</span>
                        <span>{ "s" }</span>
                    </button>
                }
            } else if item == self.page {
                html! {
                    <button class="nav-link active",
                        onclick=|_| item.clone(), >
                        { mark }
                    </button>
                }
            } else {
                html! {
                    <button class="nav-link",
                        onclick=|_| item.clone(), >
                        { mark }
                    </button>
                }
            }
        };
        // nav bar
        html! {
            <nav class="nav nav-bar", >
                { link(Page::Index) }
                { link(Page::Posts) }
                { link(Page::About) }
                { link(Page::Friend) }
            </nav>
        }
    }
}
