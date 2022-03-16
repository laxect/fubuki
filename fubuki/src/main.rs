#![recursion_limit = "256"]

mod content;
mod fetch_agent;
mod logger;
mod navbar;
mod posts;
mod style;

use content::Content;
use navbar::Navbar;
use posts::Posts;
use stylist::yew::{styled_component, use_media_query, use_style, Global};
use yew::{classes, html, use_context, ContextProvider, Html};
use yew_router::{BrowserRouter, Routable, Switch};

use crate::style::{Colors, Layout};

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
    let layout = style::Layout::leaf();
    let colors = use_context::<Colors>().unwrap();
    let class = use_style!(
        "
        font-size: 0.8rem;
        height: ${main}rem;
        padding-top: ${top}rem;
        padding-bottom: ${bottom}rem;
        &, a, a:visited {
            transition: all 0.3s;
            color: ${color};
        }
        a:hover {
            color: ${fg};
        }",
        top = layout.footer_top,
        main = layout.footer_main,
        bottom = layout.footer_bottom,
        color = colors.shadow,
        fg = colors.normal,
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
        Route::Posts => html! { <Posts /> },
        route => html! { <Content route={route.clone()} /> },
    }
}

#[styled_component(Blog)]
fn blog() -> Html {
    let colors = style::colors(style::Theme::Light);
    let is_on_small_device = use_media_query("max-width: 1036px");
    // layout
    let top = if is_on_small_device { 2.0 } else { 3.0 };
    let other = top + Layout::footer() + Layout::navbar();
    let class = classes![
        use_style!(
            "
        padding-top: ${top}em;
        min-height: calc(100vh - ${other}em);",
            top = top,
            other = other
        ),
        "heti"
    ];
    // global style
    let global = css!(
        "
        font-size: 12pt;
        color: ${fg};
        overflow-y: scroll;",
        fg = colors.normal
    );
    html! {
        <>
        <Global css={global}/>
        <ContextProvider<Colors> context={colors}>
        <BrowserRouter>
            <Navbar />
            <main {class}>
                <Switch<Route> render={Switch::render(switch)} />
            </main>
        </BrowserRouter>
            <Footer />
        </ ContextProvider<Colors>>
        </>
    }
}

fn main() {
    logger::init();
    yew::start_app::<Blog>();
}
