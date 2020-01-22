use crate::{fetch_agent::fetch, router};
use yew::*;

const VERSION_CHECK_URI: &str = "https://tenzan.gyara.moe/key/blog_version";

// return outdata: bool
pub fn check_version(tenzan: &str) -> bool {
    let build_version = semver::Version::parse(std::env!("CARGO_PKG_VERSION")).unwrap();
    let tenzan_version = semver::Version::parse(tenzan).unwrap();
    build_version < tenzan_version
}

pub enum Message {
    Click,
    VersionGet(String),
    NeverBack,
}

pub struct UpdateNotification {
    link: ComponentLink<Self>,
    router: Box<dyn Bridge<router::Router>>,
    outdate: bool,
}

impl UpdateNotification {
    fn fetch_version(&self) {
        let cb = self.link.callback(Message::VersionGet);
        let future = async move {
            if let Ok(version) = fetch::get(VERSION_CHECK_URI).await {
                cb.emit(version);
            }
        };
        wasm_bindgen_futures::spawn_local(future);
    }
}

impl Component for UpdateNotification {
    type Message = Message;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        // never callback
        let router_callback = link.callback(|_| Message::NeverBack);
        let bridge = router::Router::bridge(router_callback);
        let component = Self {
            link,
            outdate: false,
            router: bridge,
        };
        component.fetch_version();
        component
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::Click => {
                self.router.send(router::Request::Reload(true));
                false
            }
            Message::VersionGet(mut tenzan_version) => {
                if tenzan_version.starts_with('+') {
                    tenzan_version.remove(0);
                    let outdate = check_version(&tenzan_version);
                    self.outdate = outdate;
                    outdate
                } else {
                    false
                }
            }
            Message::NeverBack => unreachable!(),
        }
    }

    fn view(&self) -> Html {
        if self.outdate {
            let on_click = self.link.callback(|_| Message::Click);
            html! {
                <button onclick=on_click class="update-notification">{ "更新ある" }</button>
            }
        } else {
            html! {
                <></>
            }
        }
    }
}
