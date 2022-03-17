use stylist::yew::styled_component;
use yew::html;

#[styled_component(Loading)]
pub fn loading() -> Html {
    html! {
        {"loading"}
    }
}
