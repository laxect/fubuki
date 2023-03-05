use yew::{hook, use_effect, use_memo};

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
