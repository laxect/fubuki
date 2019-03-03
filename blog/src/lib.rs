use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

mod navbar;
use navbar::{ NavBar, Page };

pub struct Blog {
    page: Page
}

pub enum Msg {
    Click
}

impl Component for Blog {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Blog {
            page: Page::Index,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
            }
        }
        true
    }
}

impl Renderable<Blog> for Blog {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="blog", >
            <NavBar: page=Page::Index, />
            <p>{ self.page.value() }</p>
            </div>
        }
    }
}
