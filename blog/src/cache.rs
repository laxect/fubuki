use crate::{content::Msg, posts::PostList, Page};
use serde_derive::{Deserialize, Serialize};
use yew::{format::Json, services::StorageService};

#[derive(Clone, Serialize, Deserialize)]
pub enum Load {
    Page(String),
    Posts(PostList),
}

impl From<String> for Load {
    fn from(s: String) -> Load {
        Load::Page(s)
    }
}

impl From<PostList> for Load {
    fn from(pl: PostList) -> Load {
        Load::Posts(pl)
    }
}

impl From<Msg> for Load {
    fn from(m: Msg) -> Load {
        match m {
            Msg::Pong(s) => s.into(),
            Msg::Posts(pl) => pl.into(),
        }
    }
}

pub struct Cache {
    inner: StorageService,
}

impl Cache {
    pub fn new() -> Cache {
        Cache {
            inner: StorageService::new(yew::services::storage::Area::Local),
        }
    }

    pub fn get(&mut self, page: &Page) -> Option<Load> {
        let key = page.value();
        if let Json(Ok(cc)) = self.inner.restore(&key) {
            Some(cc)
        } else {
            None
        }
    }

    pub fn set(&mut self, page: &Page, content: &Load) {
        let key = page.value();
        self.inner.store(&key, Json(content));
    }
}
