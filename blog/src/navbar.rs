use crate::utils::Page;
use yew::{html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};

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
    link: ComponentLink<Self>,
}

impl Component for NavBar {
    type Message = Page;
    type Properties = NavStatus;

    fn create(prop: Self::Properties, link: ComponentLink<Self>) -> Self {
        NavBar {
            page: prop.page,
            on_change: prop.on_change,
            link
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

    fn view(&self) -> Html {
        // link item
        let link = |item: Page| -> Html {
            let mark = if item != Page::Index {
                item.value()
            } else {
                "g.s.".into()
            };
            let mut post = false;
            if let Page::Article(_) = self.page {
                post = true;
            };
            let move_item = item.clone();
            let on_click = self.link.callback(move |_| move_item.clone());
            if post && item == Page::Posts {
                html! {
                    <button class="nav-link active"
                        onclick=on_click>
                        <span class="mark">{ "post" }</span>
                        <span class="unmark">{ "s" }</span>
                    </button>
                }
            } else {
                let class = if item == Page::Index {
                    "nav-brand"
                } else if item == self.page {
                    "nav-link active current"
                } else {
                    "nav-link"
                };
                html! {
                    <button class=class
                        onclick=on_click>
                        { mark }
                    </button>
                }
            }
        };
        // nav bar
        html! {
            <>
            <nav class="nav nav-bar">
                { link(Page::Index) }
                <div class="nav-bar-right">
                    { link(Page::Posts) }
                    { link(Page::Links) }
                    { link(Page::About) }
                </div>
            </nav>
            </>
        }
    }
}
