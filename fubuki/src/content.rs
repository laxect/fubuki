use crate::{
    fetch_agent::{FetchAgent, FetchRequest, Load},
    markdown::render_markdown,
    style::Colors,
    Route,
};
use fubuki_types::{FrontMatter, Spoiler};
use stylist::yew::{styled_component, use_style};
use yew::{classes, html, use_context, use_state_eq, Html, Properties};
use yew_agent::{use_bridge, UseBridgeHandle};

#[derive(PartialEq, Clone, Properties)]
pub struct ArticleProps {
    pub page: String,
}

#[derive(Clone, PartialEq, Properties)]
struct SpoilerProps {
    spoiler: Spoiler,
}

#[styled_component(SpoilerAlert)]
fn spoiler_alert(props: &SpoilerProps) -> Html {
    let colors: Colors = use_context().unwrap();
    let SpoilerProps { spoiler } = props;
    let spoiler_alert = use_style!("color: ${fg};", fg = colors.red);
    if let Spoiler::Some { target, level } = spoiler {
        html! {
            <p class={classes![spoiler_alert, "spoiler-alert"]}>
            { format!("请注意，本文可能含有对{}的 ", target) }
            <span class={css!(font-weight: bold;)}>{ level }</span>
            { " 等级剧透。" }
            </p>
        }
    } else {
        html! {
            <></>
        }
    }
}

#[styled_component(Article)]
pub(crate) fn article(props: &ArticleProps) -> Html {
    let ArticleProps { page } = props;

    let mut main: String;
    let mut title = String::new();
    let mut spoiler = Spoiler::None;
    // remove front matter
    if page.starts_with("---\n") {
        if let Some(fm) = page.split("---\n").nth(1) {
            // ---\n..---\n
            match serde_yaml::from_str::<FrontMatter>(fm) {
                Ok(fm) => {
                    log::info!("get front_matter\n{:?}", &fm);
                    title = fm.title;
                    spoiler = fm.spoiler;
                }
                Err(e) => {
                    log::error!("fm parser failed: {}", e);
                }
            }
            main = page[fm.len() + 8..].to_string();
        } else {
            unreachable!();
        }
    } else {
        main = page.to_owned();
    }
    if main.starts_with("# ") {
        let title_end = main.find('\n').unwrap_or_default();
        title = main[..title_end].replace("# ", "");
        main = main[title_end..].to_owned();
    }
    let h1 = if title.is_empty() {
        html! { <></> }
    } else {
        html! { <h1>{ title }</h1> }
    };
    html! {
        <>
            { h1 }
            <SpoilerAlert {spoiler} />
            <article>{ render_markdown(&main) }</article>
        </>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub(crate) struct ContentProps {
    pub route: Route,
}

#[styled_component(Content)]
pub(crate) fn content(props: &ContentProps) -> Html {
    let ContentProps { route } = props;
    let page = use_state_eq(|| None);
    let handle: UseBridgeHandle<FetchAgent> = {
        let page = page.clone();
        use_bridge(move |res| match res {
            Load::Page(p) => page.set(Some(p)),
            _ => unreachable!(),
        })
    };
    handle.send(FetchRequest(route.clone()));
    if let Some(page) = (*page).clone() {
        html! {
        <Article {page} />
        }
    } else {
        html! {
        <>
            { "loading" }
        </>
        }
    }
}
