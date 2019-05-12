use crate::content::Msg;
use crate::posts::PostList;
use crate::Page;
use std::collections::HashMap;
use yew::format::Json;
use yew::services::StorageService;
use serde_derive::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum CacheContent {
    Page(String),
    Posts(PostList),
}

impl From<String> for CacheContent {
    fn from(s: String) -> CacheContent {
        CacheContent::Page(s)
    }
}

impl From<PostList> for CacheContent {
    fn from(pl: PostList) -> CacheContent {
        CacheContent::Posts(pl)
    }
}

impl From<Msg> for CacheContent {
    fn from(m: Msg) -> CacheContent {
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
            inner: StorageService::new(yew::services::storage::Area::Local)
        }
    }

    pub fn get(&mut self, page: &Page) -> Option<CacheContent> {
        let key = page.value();
        if let Json(Ok(cc)) = self.inner.restore(&key) {
            Some(cc)
        } else {
            None
        }

    }

    pub fn set(&mut self, page: Page, content: CacheContent) {
        let key = page.value();
        self.inner.store(&key, Json(&content));
    }
}
