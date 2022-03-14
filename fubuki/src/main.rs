#![recursion_limit = "256"]
#![allow(clippy::unused_unit)]

// mod content;
mod fetch_agent;
mod markdown;
mod navbar;
// mod posts;
mod style;
mod utils;

// use content::Content;
// use posts::Posts;
use navbar::Navbar;
use stylist::yew::{styled_component, use_style, Global};
use yew::{function_component, html, use_context, ContextProvider, Html};
use yew_router::{BrowserRouter, Routable, Switch};

use crate::style::Colors;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
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

const CC3: &str = "https://creativecommons.org/licenses/by-nc-sa/3.0/deed.ja";
#[styled_component(Footer)]
fn footer() -> Html {
    let layout = style::Layout::layout();
    let colors = use_context::<Colors>().unwrap();
    let class = use_style!(
        "
  font-size: 0.8rem;
  height: ${main}rem;
  padding-top: ${top}rem;
  padding-bottom: ${bottom}rem;
  &, a, a:visited {
    color: ${color};
  }
",
        top = layout.footer_top,
        main = layout.footer_main,
        bottom = layout.footer_bottom,
        color = colors.shadow
    );
    html! {
        <footer {class}>
            <p>{ "このブログ記事は" }<a href={CC3}>{ "クリエイティブ・コモンズ 表示-継承ライセンス" }</a>{ "の下で利用可能です。" }</p>
            <p>{ "メールアドレス：me at gyara dot moe。" }</p>
            <p>{ ["ビルドバージョン：", std::env!("CARGO_PKG_VERSION"), "。"].concat() }</p>
        </footer>
    }
}

fn switch(route: &Route) -> Html {
    match route {
        Route::Posts => html! { "posts" },
        _ => html! { "hello" },
    }
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
            <main>
                <Switch<Route> render={Switch::render(switch)} />
            </main>
        </BrowserRouter>
            <Footer />
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
