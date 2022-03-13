#![recursion_limit = "256"]
#![allow(clippy::unused_unit)]

mod cache;
mod content;
mod fetch_agent;
mod markdown;
mod navbar;
mod posts;
mod router;
mod style;
mod utils;

use content::Content;
use navbar::Navbar;
use stylist::yew::Global;
use utils::Page;
use yew::function_component;
use yew::html;
use yew::ContextProvider;
use yew_router::{BrowserRouter, Routable};

use crate::style::Colors;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Main,
    #[at("/posts")]
    Posts,
    #[at("/post/:id")]
    Post { id: String },
    #[at("/about")]
    About,
    #[at("/links")]
    Links,
}

#[function_component(Blog)]
fn blog() -> Html {
    let colors = style::colors(style::Theme::Light);
    html! {
        <>
        <Global css=""/>
        <ContextProvider<Colors> context={colors}>
        <BrowserRouter>
            <Navbar />
        </BrowserRouter>
            <footer>
                <p>{ "このブログ記事は" }<a href="https://creativecommons.org/licenses/by-nc-sa/3.0/deed.ja">{ "クリエイティブ・コモンズ 表示-継承ライセンス" }</a>{ "の下で利用可能です。" }</p>
                <p>{ "メールアドレス：me at gyara dot moe。" }</p>
                <p>{ ["ビルドバージョン：", std::env!("CARGO_PKG_VERSION"), "。"].concat() }</p>
            </footer>
        </ ContextProvider<Colors>>
        </>
    }
}

fn main() {
    web_logger::custom_init(web_logger::Config {
        level: log::Level::Info,
    });
    yew::start_app::<Blog>();
}
