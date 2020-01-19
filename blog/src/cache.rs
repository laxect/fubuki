use crate::{content::Msg, posts::PostList, Page};
use serde_derive::{Deserialize, Serialize};
use yew::{format::Json, services::StorageService};
use web_sys::window;

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
    pub fn check_cache_version() {
        let store = window().unwrap().local_storage().unwrap().unwrap();
        let key = "build_version";
        let version = std::env!("CARGO_PKG_VERSION").to_string();
        if let Ok(Some(cache_version)) = store.get(key) {
            if cache_version != version {
                let _ = store.clear();
                store.set(key, &version).unwrap();
            }
        } else {
            let _ = store.clear();
            store.set(key, &version).unwrap();
        }
    }

    pub fn new() -> Cache {
        Cache::check_cache_version();
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
