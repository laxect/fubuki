pub use fubuki_types::{FrontMatter, Post};

/// find front matter from content
fn find_front_matter(content: &str) -> Option<(String, String)> {
    if let Some(after) = content.strip_prefix("---\n") {
        if let Some(end) = after.find("---\n") {
            return Some((after[..end].into(), after[end + 4..].into()));
        }
    }
    None
}

/// transfer front matter from yaml to json
fn front_matter_transfer(fm_str: &str) -> Result<FrontMatter, serde_yaml::Error> {
    serde_yaml::from_str(fm_str)
}

/// parse content and return front matter json
pub fn parse_front_matter(content: String) -> anyhow::Result<(FrontMatter, String)> {
    if let Some((fm_yaml, content)) = find_front_matter(&content) {
        if let Ok(fm) = front_matter_transfer(&fm_yaml) {
            return Ok((fm, content));
        }
    }
    Err(anyhow::Error::msg("Front matter parse failed"))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn find_test() {
        let content = "---\nabc---\n";
        let fm = find_front_matter(content).unwrap();
        assert_eq!(fm.0, "abc");
    }
    #[test]
    fn transfer_test() {
        let input = r#"
            title : Goodbye Demon
            date : 2016-03-03 23:11
            category : After reading
            tags : [ 米澤 穂信, light-novel ]
            summary: 再见，妖精"#;
        assert!(front_matter_transfer(input).is_ok());
    }
    #[test]
    fn remove_time() {
        let input = r#"
            title : Goodbye Demon
            date : 令和1/5/11 23:11
            category : After reading
            tags : [ 米澤 穂信, light-novel ]
            summary: 再见，妖精"#;
        let fm = front_matter_transfer(input).unwrap();
        let mut post = fm.into_post("test".to_owned(), "".to_owned());
        post.remove_time();
        assert_eq!(post.date, "令和1/5/11");
    }
}
