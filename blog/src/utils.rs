use serde_derive::{Deserialize, Serialize};
use stdweb::unstable::TryFrom;
use yew::agent::Transferable;

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub enum Page {
    Index,
    Article(String),
    Posts,
    About,
    Links,
}

impl Page {
    pub fn title(&self) -> String {
        let mut title: String = "Gyara studio - ".into();
        match self {
            Page::Article(ref article) => {
                title.push_str(article);
            }
            _ => {
                title.push_str(&self.value());
            }
        }
        title
    }

    pub fn value(&self) -> String {
        match self {
            Page::Article(ref article) => {
                let mut post = article.clone();
                post.insert_str(0, "post/");
                post
            }
            Page::About => "about".into(),
            Page::Index => "index".into(),
            Page::Links => "links".into(),
            Page::Posts => "posts".into(),
        }
    }

    pub fn url(&self) -> String {
        let mut file = match self {
            Page::Article(ref article) => format!("/post/{}", article.clone()),
            _ => self.value(),
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

impl TryFrom<String> for Page {
    type Error = ();

    fn try_from(mut input: String) -> Result<Page, Self::Error> {
        if input.starts_with("post/") {
            return Ok(Page::Article(input.split_off(5)));
        }
        match input.as_str() {
            "" | "index" => Ok(Page::Index),
            "posts" => Ok(Page::Posts),
            "about" => Ok(Page::About),
            "links" => Ok(Page::Links),
            _ => Ok(Page::Index), // should be 404 here
        }
    }
}

impl Transferable for Page {}
