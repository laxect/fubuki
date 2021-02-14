use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

#[derive(Debug, Deserialize)]
pub struct FrontMatter {
    pub title: String,
    pub category: String,
    pub tags: Vec<String>,
    pub summary: String,
    pub date: String,
    #[serde(default)]
    pub spoiler: Spoiler,
}

impl FrontMatter {
    // will discard spoiler cause post not contain it
    pub fn into_post(self, url: String, hash: String) -> Post {
        Post {
            url,
            hash,
            title: self.title,
            date: self.date,
            summary: self.summary,
            category: self.category,
            tags: self.tags,
            spoiler: self.spoiler,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum Spoiler {
    None,
    Some { target: String, level: u32 },
}

impl Default for Spoiler {
    fn default() -> Self {
        Self::None
    }
}

#[derive(PartialEq, Clone, Deserialize, Serialize, Debug)]
pub struct Post {
    pub url: String,
    pub date: String,
    pub hash: String,
    pub title: String,
    pub summary: String,
    pub category: String,
    pub tags: Vec<String>,
    pub spoiler: Spoiler,
}

impl Post {
    pub fn remove_time(&mut self) {
        self.date = self.date.split(' ').next().unwrap().to_string();
    }

    pub fn has_spoiler(&self) -> bool {
        matches!(
            &self.spoiler,
            Spoiler::Some {
                target: _target,
                level: _level
            }
        )
    }
}

pub type PostList = Vec<Post>;

impl Default for Post {
    fn default() -> Post {
        Post {
            url: String::from(""),
            date: String::from(""),
            hash: String::from(""),
            title: String::from(""),
            summary: String::from(""),
            category: String::from(""),
            tags: Vec::new(),
            spoiler: Spoiler::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spoiler() {
        let input = r#"{"target": "test", "level": 10}"#;
        let s: Spoiler = serde_json::from_str(input).unwrap();
        assert_eq!(
            s,
            Spoiler::Some {
                target: "test".to_owned(),
                level: 10
            }
        );
    }

    #[test]
    fn time_remove() {
        let fm = FrontMatter {
            title: "title".to_owned(),
            date: "昭和11/2/26 05:00".to_owned(),
            summary: "summary".to_owned(),
            tags: vec!["tags".to_owned()],
            category: "category".to_owned(),
            spoiler: Spoiler::None,
        };
        let mut post = fm.into_post("https://example.com".to_owned(), "".to_owned());
        post.remove_time();
        assert_eq!(post.date, "昭和11/2/26");
    }

    #[test]
    fn time_remove_2() {
        let fm = FrontMatter {
            title: "title".to_owned(),
            date: String::new(),
            summary: "summary".to_owned(),
            tags: vec!["tags".to_owned()],
            category: "category".to_owned(),
            spoiler: Spoiler::None,
        };
        let mut post = fm.into_post("https://example.com".to_owned(), "".to_owned());
        post.remove_time();
        assert_eq!(post.date, "");
    }
}
