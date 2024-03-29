use crate::{
    style::Colors,
    utils::{use_remote, use_title},
    Route,
};
use fubuki_types::{FrontMatter, Spoiler};
use stylist::yew::{styled_component, use_style};
use yew::{classes, html, use_context, virtual_dom::VNode, Html, HtmlResult, Properties};

mod style;
mod webmention;

#[derive(PartialEq, Clone, Properties)]
pub struct ArticleProps {
    pub route: Route,
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
pub(crate) fn article(props: &ArticleProps) -> HtmlResult {
    let ArticleProps { route } = props;

    let page = use_remote(route.into_url())?;
    let is_post = route.is_post();
    let render_title = matches!(route, Route::Post { .. });

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
    Ok(html! {
        <article {class}>
            { h1 }
            <SpoilerAlert {spoiler} />
            { render_markdown(&main) }
            if is_post {
                <webmention::Echo />
            }
        </article>
    })
}
