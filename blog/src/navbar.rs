use crate::utils::Page;
use yew::{html, Callback, Component, ComponentLink, Html, Properties, Renderable, ShouldRender};

#[derive(PartialEq, Clone, Properties)]
pub struct NavStatus {
    #[props(required)]
    pub page: Page,
    #[props(required)]
    pub on_change: Callback<Page>,
}

pub struct NavBar {
    page: Page,
    on_change: Callback<Page>,
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
            self.on_change.emit(msg);
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
                    <button class="nav-link active"
                        onclick=|_| item.clone()>
                        <span class="mark">{ "post" }</span>
                        <span class="unmark">{ "s" }</span>
                    </button>
                }
            } else if item == self.page {
                html! {
                    <button class="nav-link active current"
                        onclick=|_| item.clone()>
                        { mark }
                    </button>
                }
            } else {
                html! {
                    <button class="nav-link"
                        onclick=|_| item.clone()>
                        { mark }
                    </button>
                }
            }
        };
        // nav bar
        html! {
            <nav class="nav nav-bar">
                { link(Page::Index) }
                { link(Page::Posts) }
                { link(Page::Links) }
                { link(Page::About) }
            </nav>
        }
    }
}
