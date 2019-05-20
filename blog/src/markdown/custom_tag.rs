#[derive(Debug, PartialEq)]
pub enum HTag {
    Left(String),
    Right(String),
    Text(String),
}

impl HTag {
    pub fn from_string(html_tag: &str) -> HTag {
        if html_tag.starts_with("</") {
            HTag::Right(String::from(&html_tag[1..html_tag.len() - 1]))
        } else {
            HTag::Left(String::from(&html_tag[1..html_tag.len() - 1]))
        }
    }
}

pub fn html_parse(html: &str) -> Vec<HTag> {
    let mut h_tags = Vec::new();
    for tom in html.split('>') {
        if tom.starts_with("</") {
            h_tags.push(HTag::Right(String::from(&tom[2..])));
        } else if tom.starts_with('<') {
            h_tags.push(HTag::Left(String::from(&tom[1..])));
        } else if let Some(ind) = tom.find('<') {
            h_tags.push(HTag::Text(String::from(&tom[..ind])));
            if tom[ind..].starts_with("</") {
                h_tags.push(HTag::Right(String::from(&tom[ind + 2..])));
            } else {
                h_tags.push(HTag::Left(String::from(&tom[ind + 1..])));
            }
        } else {
            h_tags.push(HTag::Text(String::from(tom)));
        }
    }
    h_tags
}
