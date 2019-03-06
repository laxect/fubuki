#[derive(PartialEq, Clone, Copy)]
pub enum Page {
    Index,
    Article(&'static str),
    About,
    Friend,
}

impl Page {
    pub fn value(&self) -> &'static str {
        match self {
            Page::Index => "index",
            Page::Article(_) => "post",
            Page::About => "about",
            Page::Friend => "Friends",
        }
    }
}

