mod cache;
mod content;
mod fetch_agent;
mod markdown;
mod navbar;
mod posts;
mod router;
mod utils;

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
    router: Box<dyn Bridge<Router>>,
    link: ComponentLink<Self>
}

impl Component for Blog {
    type Message = Change;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let cb = link.callback(Change::NavTo);
        let mut router = Router::bridge(cb);
        router.send(Request::Where);
        Blog {
            page: Page::Index,
            router,
            link,
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

    fn view(&self) -> Html {
        let on_change = self.link.callback(Change::Click);
        html! {
            <>
                <NavBar: page=self.page.clone()
                    on_change=on_change.clone()/>
                <Content: page=self.page.clone()
                    on_change=on_change/>
                <footer>
                    <p>{ "このブログ記事は" }<a href="https://creativecommons.org/licenses/by-nc-sa/3.0/deed.ja">{ "クリエイティブ・コモンズ 表示-継承ライセンス" }</a>{ "の下で利用可能です" }</p>
                </footer>
            </>
        }
    }
}
