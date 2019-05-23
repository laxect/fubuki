#[derive(Debug, PartialEq)]
pub enum HTag {
    Left(String),
    Right(String),
    Text(String),
}

impl HTag {
    pub fn from_str(html: &str) -> HTag {
        let inner = html.replace(|ch: char| !ch.is_ascii_alphabetic(), "");
        if html.starts_with("</") {
            HTag::Right(inner)
        } else if html.starts_with('<') {
            HTag::Left(inner)
        } else {
            // no filter in text
            HTag::Text(html.to_string())
        }
    }
}

pub fn html_parse(html: &str) -> Vec<HTag> {
    let mut h_tags = Vec::new();
    for tom in html.split('>') {
        if tom.starts_with('<') {
            h_tags.push(HTag::from_str(tom));
        } else if let Some(ind) = tom.find('<') {
            h_tags.push(HTag::from_str(&tom[..ind]));
            h_tags.push(HTag::from_str(&tom[ind..]));
        } else {
            h_tags.push(HTag::from_str(tom));
        }
    }
    h_tags
}
