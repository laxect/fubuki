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

#[derive(Debug, Deserialize, Serialize, PartialEq)]
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
    pub title: String,
    pub summary: String,
    pub category: String,
    pub tags: Vec<String>,
}

pub type PostList = Vec<Post>;

impl Default for Post {
    fn default() -> Post {
        Post {
            url: String::from(""),
            date: String::from(""),
            title: String::from(""),
            summary: String::from(""),
            category: String::from(""),
            tags: Vec::new(),
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
}
