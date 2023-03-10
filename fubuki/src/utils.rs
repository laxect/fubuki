use yew::{
    hook,
    suspense::{use_future, SuspensionResult},
    use_effect, use_memo,
};

#[hook]
pub fn use_title<T: Into<String>>(title: T) {
    let title = title.into();
    let pre_title = use_memo(|_| gloo_utils::document().title(), ());

    use_effect(move || {
        if !title.is_empty() {
            let title = format!("島風造船所 - {}", title);
            if gloo_utils::document().title() != title {
                gloo_utils::document().set_title(&title);
            }
        }
        move || {
            gloo_utils::document().set_title(&*pre_title);
        }
    });
}

#[hook]
pub fn use_remote<T>(target: T) -> SuspensionResult<String>
where
    T: AsRef<str> + 'static,
{
    let res = use_future(|| async move { gloo_net::http::Request::get(target.as_ref()).send().await?.text().await })?;
    match *res {
        Ok(ref res) => Ok(res.to_owned()),
        Err(ref e) => Ok(e.to_string()),
    }
}

#[hook]
pub fn use_json<T, J>(target: T) -> Option<J>
where
    T: AsRef<str> + 'static,
    J: serde::de::DeserializeOwned + 'static,
{
    let text = use_remote(target).ok()?;
    serde_yaml::from_str(&text).ok()
}
