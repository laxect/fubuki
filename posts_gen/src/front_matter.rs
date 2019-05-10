use serde_derive::{Deserialize, Serialize};

/// instead of a front matter
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct FrontMatter {
    url: Option<String>,
    title: String,
    date: String,
    summary: String,
    tags: Vec<String>,
    category: String,
}

impl FrontMatter {
    pub fn fill_url(&mut self, url: String) {
        self.url = Some(url);
    }

    pub fn get_url(&self) -> &str {
        match self.url {
            Some(ref u) => u,
            None => "",
        }
    }
}

/// find front matter from content
fn find_front_matter(content: String) -> Option<String> {
    if content.starts_with("---\n") {
        let after = &content[4..];
        if let Some(end) = after.find("---\n") {
            return Some(after[..end].into());
        }
    }
    None
}

/// transfer front matter from yaml to json
fn front_matter_transfer(fm_str: String) -> Result<FrontMatter, serde_yaml::Error> {
    serde_yaml::from_str(&fm_str)
}

/// parse content and return front matter json
pub fn parse_front_matter(content: String) -> Option<FrontMatter> {
    if let Some(fm_yaml) = find_front_matter(content) {
        match front_matter_transfer(fm_yaml) {
            Ok(fm) => {
                println!("    O parser passed");
                return Some(fm);
            }
            Err(e) => println!("    E {}", e),
        }
    }
    println!("    X parser failed");
    None
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn find_test() {
        let content = "---\nabc---\n".to_string();
        let fm = find_front_matter(content);
        assert_eq!(fm, Some("abc".to_string()));
    }
    #[test]
    fn transfer_test() {
        let input: String = r#"
            title : Goodbye Demon
            date : 2016-03-03 23:11
            category : After reading
            tags : [ 米澤 穂信, light-novel ]
            summary: 再见，妖精"#
            .into();
        assert!(front_matter_transfer(input).is_ok());
    }
}
