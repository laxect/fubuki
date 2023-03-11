use serde::{Deserialize, Serialize};
use stylist::yew::styled_component;
use yew::{html, Html};

use crate::utils::use_json;

#[derive(Serialize, Deserialize, PartialEq, Clone)]
struct Webmention {
    url: String,
}

#[derive(Serialize, Deserialize, PartialEq)]
struct Webmentions {
    children: Vec<Webmention>,
}

fn get_current() -> Option<String> {
    let current = gloo_utils::document().location()?;
    let url = format!("{}{}", current.origin().ok()?, current.pathname().ok()?);
    log::info!("current at: {url}");

    Some(url)
}

const TOKEN: &str = "iLobGtxghdo0MnNFqW7bbA";

#[styled_component(Echo)]
pub(super) fn echo() -> Html {
    let url = get_current().unwrap();
    let mentions: Vec<Webmention> = use_json(format!(
        "https://webmention.io/api/mentions.jf2?target={url}&token={TOKEN}"
    ))
    .unwrap_or_default();

    if mentions.is_empty() {
        return html! { <></> };
    }
    let mentions_list = mentions
        .into_iter()
        .map(|m| html! {<li><a href={ m.url.clone() }> { m.url } </a></li>})
        .collect::<Html>();
    html! {
        <div>
            <h3>{ "Echo" }</h3>
            <ul>
            { mentions_list }
            </ul>
        </div>
    }
}
