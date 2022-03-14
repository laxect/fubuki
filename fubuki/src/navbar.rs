use crate::{style::Colors, Route};
use stylist::{
    css,
    yew::{styled_component, use_style},
};
use yew::{classes, html, use_context, Callback, Properties};
use yew_router::{
    history::History,
    hooks::{use_history, use_route},
};

#[derive(PartialEq, Properties)]
struct ButtonProps {
    item: Route,
    ima: Route,
}

#[styled_component(Button)]
fn button(props: &ButtonProps) -> Html {
    let ButtonProps { item, ima } = props;
    let history = use_history().unwrap();
    let colors: Colors = use_context().unwrap();
    let mark = match item {
        Route::Main => "🏗",
        Route::Posts => "posts",
        Route::About => "about",
        Route::Links => "links",
        _ => unreachable!(),
    };
    let button_common = use_style!(
        "
         text-align: center;
         padding: 0.4rem 0;
         transition-property: all;
         transition-duration: 0.3s;
         transition-timing-function: ease-out;"
    );
    let style = match (item, ima) {
        (&Route::Main, _) => use_style!(
            "
             padding: 0.4rem;
             background-color: ${bg1};
             color: transparent;
             text-shadow: 0 0 0 ${fg1};
             &:hover {
               text-shadow: 0 0 0 ${fg2};
             }",
            bg1 = colors.brand_bg1,
            fg1 = colors.brand_fg1,
            fg2 = colors.brand_fg2
        ),
        (a, b) => {
            if a == b || (matches!(ima, Route::Post { id: _ }) && *item == Route::Posts) {
                use_style!(
                    "background-color: ${bg}; color: ${fg};",
                    bg = colors.rev_bg,
                    fg = colors.rev_fg
                )
            } else {
                use_style!("&:hover { background-color: ${bg};}", bg = colors.bg2)
            }
        }
    };
    let pointer = if item == ima {
        use_style!(pointer-events: none;)
    } else {
        use_style!(pointer-events: auto;)
    };
    let ii = item.clone();
    let onclick = Callback::once(move |_| history.push(ii.clone()));
    let class = classes![button_common, style, pointer];
    if matches!(ima, Route::Post { id: _ }) && *item == Route::Posts {
        let shadow = use_style!("color: ${fg};", fg = colors.rev_shadow);
        html! {
            <button {class} {onclick}>
                <span>{ "post" }</span>
                <span class={shadow}>{ "s" }</span>
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

#[styled_component(Navbar)]
pub fn navbar() -> Html {
    let ima: Route = use_route().unwrap();
    let navbar = use_style!(r#"font-family: "Iosevka Fixed SS10 web"; margin-top: 1rem;"#);
    let nav_right = use_style!(
        "--nav-item-width: 3.6rem;
         --nav-gap: 0.1rem;
         display: grid;
         grid-template-columns: repeat(3, var(--nav-item-width));
         grid-gap: var(--nav-gap);
         width: calc(var(--nav-item-width) * 3 + var(--nav-gap) * 2);
         right: 0;
         float: right;",
    );
    let site_title = use_style!("margin-left: 0.4rem; font-weight: bold;");
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
