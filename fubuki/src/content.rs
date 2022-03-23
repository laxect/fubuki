use crate::{
    fetch_agent::{FetchAgent, FetchRequest, Load},
    loading::Loading,
    style::Colors,
    utils::use_title,
    Route,
};
use fubuki_types::{FrontMatter, Spoiler};
use stylist::yew::{styled_component, use_style};
use yew::{classes, html, use_context, use_state_eq, virtual_dom::VNode, Html, Properties};
use yew_agent::{use_bridge, UseBridgeHandle};

mod style;

#[derive(PartialEq, Clone, Properties)]
pub struct ArticleProps {
    pub page: String,
    pub render_title: bool,
}

#[derive(Clone, PartialEq, Properties)]
struct SpoilerProps {
    spoiler: Spoiler,
}

fn render_markdown(md: &str) -> Html {
    let options = pulldown_cmark::Options::all();
    let parser = pulldown_cmark::Parser::new_ext(md, options);
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);
    let node = gloo_utils::document().create_element("section").unwrap();
    node.set_inner_html(&html);
    VNode::VRef(node.into())
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
    let ArticleProps { page, render_title } = props;

    let mut title = String::new();
    let mut spoiler = Spoiler::None;
    // remove front matter
    let mut main = if page.starts_with("---\n") {
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
            page[fm.len() + 8..].to_owned()
        } else {
            // guard
            unreachable!();
        }
    } else {
        page.to_owned()
    };

    if main.starts_with("# ") {
        let title_end = main.find('\n').unwrap_or_default();
        title = main[..title_end].replace("# ", "");
        main = main[title_end..].to_owned();
    }

    let title_style = use_style!("margin-bottom: 2rem; margin-top: 0;");
    use_title(title.clone());
    let h1 = if title.is_empty() || !render_title {
        html! { <></> }
    } else {
        html! { <h1 class={title_style}>{ title }</h1> }
    };

    let colors: Colors = use_context().unwrap();
    let class = use_style(style::article(&colors));
    html! {
        <article {class}>
            { h1 }
            <SpoilerAlert {spoiler} />
            { render_markdown(&main) }
        </article>
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

    let render_title = matches!(route, Route::Post { .. });
    if let Some(page) = (*page).clone() {
        html! {
        <Article {page} {render_title} />
        }
    } else {
        html! {
        <Loading />
        }
    }
}
