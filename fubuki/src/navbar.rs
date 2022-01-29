use crate::utils::Page;
use yew::{html, Callback, Component, Context, Html, Properties};

#[derive(PartialEq, Clone, Properties)]
pub struct NavStatus {
    pub page: Page,
    pub on_change: Callback<Page>,
}

fn get_item_mark(item: &Page) -> String {
    match item {
        Page::Index => "🏗".to_string(),
        _ => item.value(),
    }
}

pub struct NavBar {
    page: Page,
}

impl NavBar {
    fn get_callback<T>(&self, item: &Page, ctx: &Context<Self>) -> Callback<T> {
        let move_item = item.clone();
        ctx.link().callback(move |_| move_item.clone())
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

    fn link_html(&self, item: Page, ctx: &Context<Self>) -> Html {
        let mark = get_item_mark(&item);
        let onclick = self.get_callback(&item, ctx);
        if self.page.is_article() && item == Page::Posts {
            html! {
                <button class="nav-link active" {onclick}>
                    <span class="mark">{ "post" }</span>
                    <span class="unmark">{ "s" }</span>
                </button>
            }
        } else {
            let class = self.get_item_class(&item);
            html! {
                <button {class} {onclick}>
                    { mark }
                </button>
            }
        }
    }
}

impl Component for NavBar {
    type Message = Page;
    type Properties = NavStatus;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            page: ctx.props().page.clone(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        if msg != self.page {
            self.page = msg.clone();
            ctx.props().on_change.emit(msg);
            true
        } else {
            false
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.page = ctx.props().page.clone();
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // nav bar
        html! {
            <>
            <nav class="nav nav-bar">
                { self.link_html(Page::Index, ctx) }
                <span class="site-title">{ "島風造船所" }</span>
                <div class="nav-bar-right">
                    { self.link_html(Page::Posts, ctx) }
                    { self.link_html(Page::Links, ctx) }
                    { self.link_html(Page::About, ctx) }
                </div>
            </nav>
            </>
        }
    }
}
