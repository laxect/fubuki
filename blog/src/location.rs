/// give a new method to stdweb::web::location
use stdweb::{js, web::Location, JsSerialize};

pub trait LocationExt: JsSerialize {
    fn reload(&self, forced_reload: bool) {
        js! {
            @(no_return)
            @{self}.reload(@{forced_reload});
        }
    }
}

impl LocationExt for Location {}
