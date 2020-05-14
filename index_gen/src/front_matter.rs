pub use fubuki_types::{FrontMatter, Post};

pub fn front_matter_to_post(fm: FrontMatter, uri: String) -> Post {
    let FrontMatter {
        title,
        category,
        tags,
        summary,
        date,
        spoiler: _,
    } = fm;
    Post {
        url: uri,
        title,
        date,
        summary,
        category,
        tags,
    }
}

pub fn front_matter_time_remove(fm: &mut Post) {
    let ds: Vec<&str> = fm.date.split(' ').collect();
    fm.date = ds[0].to_string();
}

/// find front matter from content
fn find_front_matter(content: String) -> Option<(String, String)> {
    if content.starts_with("---\n") {
        let after = &content[4..];
        if let Some(end) = after.find("---\n") {
            return Some((after[..end].into(), after[end + 4..].into()));
        }
    }
    None
}

/// transfer front matter from yaml to json
fn front_matter_transfer(fm_str: String) -> Result<FrontMatter, serde_yaml::Error> {
    serde_yaml::from_str(&fm_str)
}

/// parse content and return front matter json
pub fn parse_front_matter(content: String) -> Option<(FrontMatter, String)> {
    if let Some((fm_yaml, content)) = find_front_matter(content) {
        match front_matter_transfer(fm_yaml) {
            Ok(fm) => {
                println!("    O parser passed");
                return Some((fm, content));
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
        let fm = find_front_matter(content).unwrap();
        assert_eq!(fm.0, "abc".to_string());
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
    #[test]
    fn remove_time() {
        let input: String = r#"
            title : Goodbye Demon
            date : 令和1/5/11 23:11
            category : After reading
            tags : [ 米澤 穂信, light-novel ]
            summary: 再见，妖精"#
            .into();
        let fm = front_matter_transfer(input).unwrap();
        let mut post = front_matter_to_post(fm, "test".to_owned());
        front_matter_time_remove(&mut post);
        assert_eq!(post.date, "令和1/5/11".to_string());
    }
}
