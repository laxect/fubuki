mod cache;
mod posts;
mod utils;
mod router;
mod navbar;
mod content;
mod markdown;
mod fetch_agent;

use content::Content;
use navbar::NavBar;
use router::{Request, Router};
use utils::Page;
use yew::*;

pub enum Change {
    Click(Page),
    NavTo(Page),
}

pub struct Blog {
    page: Page,
    router: Box<Bridge<Router>>,
}

impl Component for Blog {
    type Message = Change;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let cb = link.send_back(Change::NavTo);
        let mut router = Router::bridge(cb);
        router.send(Request::Where);
        Blog {
            page: Page::Index,
            router,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let inner = match msg {
            Change::Click(page) => {
                if page != self.page {
                    self.router.send(Request::Goto(page.clone()));
                }
                page
            }
            Change::NavTo(page) => page,
        };
        if inner != self.page {
            self.page = inner;
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
                    on_change=Change::Click, />
                <Content: page=self.page.clone(),
                    on_change=Change::Click, />
                <footer>
                    <p>{ "このブログ記事は " }<a href="https://creativecommons.org/licenses/by-nc-sa/3.0/deed.ja",>{ "CC BY-NC-SA 3.0" }</a>{ " 契約の下でライセンスされています。" }</p>
                </footer>
            </>
        }
    }
}
