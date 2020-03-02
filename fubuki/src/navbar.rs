use crate::utils::Page;
use yew::{html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(PartialEq, Clone, Properties)]
pub struct NavStatus {
    pub page: Page,
    pub on_change: Callback<Page>,
}

fn get_item_mark(item: &Page) -> String {
    match item {
        Page::Index => "g.s.".to_string(),
        _ => item.value(),
    }
}

pub struct NavBar {
    page: Page,
    on_change: Callback<Page>,
    link: ComponentLink<Self>,
}

impl NavBar {
    fn get_callback<T>(&self, item: &Page) -> Callback<T> {
        let move_item = item.clone();
        self.link.callback(move |_| move_item.clone())
    }

    fn get_item_class(&self, item: &Page) -> &'static str {
        if *item == Page::Index {
            "nav-brand"
        } else if *item == self.page {
            "nav-link active current"
        } else {
            "nav-link"
        }
    }

    fn link_html(&self, item: Page) -> Html {
        let mark = get_item_mark(&item);
        let on_click = self.get_callback(&item);
        if self.page.is_article() && item == Page::Posts {
            html! {
                <button class="nav-link active"
                    onclick=on_click>
                    <span class="mark">{ "post" }</span>
                    <span class="unmark">{ "s" }</span>
                </button>
            }
        } else {
            let class = self.get_item_class(&item);
            html! {
                <button class=class
                    onclick=on_click>
                    { mark }
                </button>
            }
        }
    }
}

impl Component for NavBar {
    type Message = Page;
    type Properties = NavStatus;

    fn create(prop: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            page: prop.page,
            on_change: prop.on_change,
            link,
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
        // nav bar
        html! {
            <>
            <nav class="nav nav-bar">
                { self.link_html(Page::Index) }
                <div class="nav-bar-right">
                    { self.link_html(Page::Posts) }
                    { self.link_html(Page::Links) }
                    { self.link_html(Page::About) }
                </div>
            </nav>
            </>
        }
    }
}
