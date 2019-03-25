use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

mod content;
mod markdown;
mod navbar;
mod posts;
mod utils;

use content::Content;
use navbar::NavBar;
use utils::Page;

pub struct Blog {
    page: Page,
}

impl Component for Blog {
    type Message = Page;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Blog { page: Page::Index }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        if msg != self.page {
            self.page = msg;
            true
        } else {
            false
        }
    }
}

impl Renderable<Blog> for Blog {
    fn view(&self) -> Html<Self> {
        html! {
            <>
                <NavBar: page=self.page.clone(),
                    on_change=|msg| msg, />
                <Content: page=self.page.clone(),
                    on_change=|msg| msg, />
            </>
        }
    }
}
