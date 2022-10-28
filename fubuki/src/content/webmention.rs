use crate::fetch_agent::{FetchAgent, FetchRequest, Response};
use serde::{Deserialize, Serialize};
use stylist::yew::styled_component;
use yew::{html, use_state_eq, Html};
use yew_agent::{use_bridge, UseBridgeHandle};

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
    let mentions = use_state_eq(|| None);

    let handle: UseBridgeHandle<FetchAgent> = {
        let mentions = mentions.clone();
        use_bridge(move |res| match res {
            Response::JSON(json) => {
                let mention_io: serde_yaml::Result<Webmentions> = serde_yaml::from_value(json);
                if let Ok(mention_io) = mention_io {
                    mentions.set(Some(mention_io.children));
                }
            }
            _ => unreachable!(),
        })
    };

    let mentions = (*mentions).clone().unwrap_or_default();
    if mentions.is_empty() {
        if let Some(url) = get_current() {
            handle.send(FetchRequest::Outer(format!(
                "https://webmention.io/api/mentions.jf2?target={url}&token={TOKEN}"
            )));
        }
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
