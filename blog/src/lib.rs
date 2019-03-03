use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

mod navbar;
use navbar::{ NavBar, Page };

pub struct Blog {
}

pub enum Msg {
    Click
}

impl Component for Blog {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Blog {}
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
            <p>{ "hello world" }</p>
            </div>
        }
    }
}
