use crate::{posts::PostList, Page};
use serde_derive::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use web_sys::{window, Storage};

#[derive(Clone, Serialize, Deserialize)]
pub enum Load {
    Page(String),
    Posts(PostList),
}

const BUILD_VERSION: &str = git_version::git_version!(args = ["--always"]);

impl Load {
    pub fn into_page(self) -> Option<String> {
        match self {
            Self::Page(page) => Some(page),
            _ => None,
        }
    }
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

fn get_local_storage() -> Result<Storage, JsValue> {
    let window = window().ok_or_else(|| JsValue::from_str("window open filed"))?;
    let store = window
        .local_storage()?
        .ok_or_else(|| JsValue::from_str("local_storage not support"))?;
    Ok(store)
}

pub struct Cache {
    inner: Storage,
}

impl Cache {
    fn check_cache_version(&self) {
        let key = "build_version";
        if let Ok(Some(cache_version)) = self.inner.get(key) {
            if cache_version == BUILD_VERSION {
                // no more action
                return;
            }
        }
        self.clear();
    }

    pub fn new() -> Cache {
        let cache = Cache {
            inner: get_local_storage().expect("cache open failed"),
        };
        cache.check_cache_version();
        cache
    }

    fn clear(&self) {
        let key = "build_version";
        let _ = self.inner.clear();
        let _ = self.inner.set(key, BUILD_VERSION);
    }

    pub fn get(&self, page: &Page) -> Option<Load> {
        // not cache on index
        let key = page.value();
        if let Ok(Some(cc)) = self.inner.get(&key) {
            if let Ok(load) = serde_yaml::from_str(&cc) {
                return Some(load);
            } else {
                self.clear(); // remove cache
            }
        }
        None
    }

    /// store index only for check update
    pub fn set(&mut self, page: &Page, content: &Load) {
        let key = page.value();
        let val = serde_yaml::to_string(content).expect("never failed");
        self.inner.set(&key, &val).ok();
    }
}
