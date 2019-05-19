#[derive(Debug, PartialEq)]
pub struct Tag {
    name: String,
    text: String,
}

impl Tag {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn text(&self) -> String {
        self.text.clone()
    }
}

pub fn parse_custom_tag(text: &str) -> Option<Tag> {
    if text.starts_with("<") {
        if let Some(tag_l_end) = text.find('>') {
            if let Some(tag_r_start) = text[1..].find('<') {
                return Some(Tag {
                    name: text[1..tag_l_end].to_string(),
                    text: text[tag_l_end + 1..tag_r_start + 1].to_string(),
                });
            }
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn parser() {
        let text = "<ins>aaa</ins>";
        let tag = parse_custom_tag(text);
        assert_eq!(
            tag,
            Some(Tag {
                name: "ins".to_string(),
                text: "aaa".to_string(),
            })
        );
    }
}
