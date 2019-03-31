use yew::agent::Transferable;
use serde_derive::{ Serialize, Deserialize };

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub enum Page {
    Index,
    Article(String),
    Posts,
    About,
    Friend,
}

impl Page {
    pub fn value(&self) -> &'static str {
        match self {
            Page::Index => "index",
            Page::Article(_) => "post",
            Page::Posts => "posts",
            Page::About => "about",
            Page::Friend => "friends",
        }
    }

    pub fn url(&self) -> String {
        let mut file = match self {
            Page::Article(ref article) => format!("/post/{}", article.clone()),
            _ => String::from(self.value()),
        };
        file.push_str(".md");
        file
    }

    pub fn is_cacheable(&self) -> bool {
        match self {
            Page::Posts => false,
            _ => true,
        }
    }
}

impl Transferable for Page {}
