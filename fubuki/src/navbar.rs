use crate::Route;
use yew::{function_component, html, Callback, Html};
use yew_router::{
    history::{AnyHistory, History},
    hooks::{use_history, use_route},
};

fn link_html(item: Route, ima: &Route, history: AnyHistory) -> Html {
    let mark = match item {
        Route::Main => "🏗",
        Route::Posts => "posts",
        Route::About => "about",
        Route::Links => "links",
        _ => unreachable!(),
    };
    let class = match (&item, ima) {
        (&Route::Main, _) => "nav-brand",
        (&Route::Post { id: _ }, &Route::Posts) => "nav-link active",
        (a, b) => {
            if a == b {
                "nav-link active"
            } else {
                "nav-link"
            }
        }
    };
    let ii = item.clone();
    let onclick = Callback::from(move |_| history.push(ii.clone()));
    if matches!(item, Route::Post { id: _ }) && *ima == Route::Posts {
        html! {
            <button {class} {onclick}>
                <span class="mark">{ "post" }</span>
                <span class="unmark">{ "s" }</span>
            </button>
        }
    } else {
        html! {
            <button {class} {onclick}>
                { mark }
            </button>
        }
    }
}

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let ima: Route = use_route().unwrap();
    let history = use_history().unwrap();
    // nav bar
    html! {
        <nav class="nav nav-bar">
            { link_html(Route::Main, &ima, history.clone()) }
            <span class="site-title">{ "島風造船所" }</span>
            <div class="nav-bar-right">
                { link_html(Route::Posts, &ima, history.clone()) }
                { link_html(Route::Links, &ima, history.clone()) }
                { link_html(Route::About, &ima, history.clone()) }
            </div>
        </nav>
    }
}
