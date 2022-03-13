use crate::Route;
use stylist::{
    css,
    yew::{styled_component, use_style},
};
use yew::{classes, html, Callback, Properties};
use yew_router::{
    history::History,
    hooks::{use_history, use_route},
};

fn nav_style(item: &Route, ima: &Route) -> &'static str {
    match (item, ima) {
        (&Route::Main, _) => {
            r#"
text-align: center;
padding: 0.4em;
background-color: var(--shironezumi);
color: transparent;
text-shadow: 0 0 0 var(--shironeri);
&:hover {
text-shadow: 0 0 0 var(--hai);
}"#
        }
        (a, b) => {
            if a == b {
                "
text-align: center;
padding: 0.4em 0;
background-color: var(--black);
color: white;
"
            } else {
                "
text-align: center;
padding: 0.4em 0;
transition-property: all;
transition-duration: 0.3s;
transition-timing-function: ease-out;
&:hover {
      background-color: var(--gray);
    }
"
            }
        }
    }
}

#[derive(PartialEq, Properties)]
struct ButtonProps {
    item: Route,
    ima: Route,
}

#[styled_component(Button)]
fn button(props: &ButtonProps) -> Html {
    let ButtonProps { item, ima } = props;
    let history = use_history().unwrap();
    let mark = match item {
        Route::Main => "🏗",
        Route::Posts => "posts",
        Route::About => "about",
        Route::Links => "links",
        _ => unreachable!(),
    };
    let style = use_style(nav_style(item, ima));
    let ii = item.clone();
    let onclick = Callback::from(move |_| history.push(ii.clone()));
    if matches!(item, Route::Post { id: _ }) && *ima == Route::Posts {
        html! {
            <button class={style} {onclick}>
                <span class={css!(background-color: var(--black); color: white;)}>{ "post" }</span>
                <span class={css!(color: var(--little_black);)}>{ "s" }</span>
            </button>
        }
    } else {
        html! {
            <button class={style} {onclick}>
                { mark }
            </button>
        }
    }
}

#[styled_component(Navbar)]
pub fn navbar() -> Html {
    let ima: Route = use_route().unwrap();
    let navbar = use_style!(
        r#"font-family: "Iosevka Fixed SS10 web";
           margin-top: 1em;"#
    );
    let nav_right = use_style!(
        "--nav-item-width: 3.6em;
         --nav-gap: 0.1em;
         display: grid;
         grid-template-columns: repeat(3, var(--nav-item-width));
         grid-gap: var(--nav-gap);
         width: calc(var(--nav-item-width) * 3 + var(--nav-gap) * 2);
         right: 0;
         float: right;",
    );
    let site_title = use_style!(
        "margin-left: 0.4em;
         font-weight: bold;"
    );
    html! {
        <nav class={classes!(navbar, "navbar")}>
            <Button item={Route::Main} ima={ima.clone()} />
            <span class={site_title}>{ "島風造船所" }</span>
            <div class={nav_right}>
                <Button item={Route::Posts} ima={ima.clone()} />
                <Button item={Route::Links} ima={ima.clone()} />
                <Button item={Route::About} ima={ima.clone()} />
            </div>
        </nav>
    }
}
