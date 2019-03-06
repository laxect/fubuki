use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};
use yew::services::ConsoleService;

mod utils;
mod navbar;
mod content;
use utils::Page;
use navbar::NavBar;

pub struct Blog {
    page: Page,
    console: ConsoleService,
}

impl Component for Blog {
    type Message = Page;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Blog {
            page: Page::Index,
            console: ConsoleService::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.console.log("out");
        self.console.log(msg.value());
        self.console.log(self.page.value());
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
                <NavBar: page=self.page,
                    on_change=|msg| msg, />
                <p>{ self.page.value() }</p>
            </>
        }
    }
}
