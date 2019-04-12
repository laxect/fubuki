use crate::content::Msg;
use crate::posts::PostList;
use crate::Page;
use std::collections::HashMap;

#[derive(Clone)]
pub enum CacheContent {
    Page(String),
    Posts(PostList),
}

pub struct Cache {
    inner: HashMap<String, CacheContent>,
}

impl Cache {
    pub fn new() -> Cache {
        Cache {
            inner: HashMap::new(),
        }
    }

    pub fn has(&self, page: &Page) -> bool {
        self.inner.contains_key(&page.value())
    }

    pub fn get(&self, page: &Page) -> Option<CacheContent> {
        self.inner.get(&page.value()).cloned()
    }

    pub fn set(&mut self, page: Page, content: CacheContent) {
        self.inner.insert(page.value(), content);
    }
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
