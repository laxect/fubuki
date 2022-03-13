#[derive(PartialEq, Clone, Debug)]
pub(crate) struct Colors {
    pub background: &'static str,
    pub bg2: &'static str,
    pub brand_bg1: &'static str,
    pub brand_bg2: &'static str,
    pub brand_fg1: &'static str,
    pub brand_fg2: &'static str,
    pub rev_bg: &'static str,
    pub rev_fg: &'static str,
    pub rev_shadow: &'static str,
    pub bold: &'static str,
    pub normal: &'static str,
    pub shadow: &'static str,
    pub underground: &'static str,
    pub red: &'static str,
    pub blue: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Theme {
    Light,
    Dark,
}

pub(crate) fn colors(theme: Theme) -> Colors {
    match theme {
        Theme::Light => Colors {
            background: "#f3f3f3",
            bg2: "#d4d4d4",
            brand_bg1: "#d4d4d4",
            brand_bg2: "#606060",
            brand_fg1: "#f3f3f3",
            brand_fg2: "#000000",
            rev_bg: "#404040",
            rev_fg: "#f3f3f3",
            rev_shadow: "#505050",
            bold: "#404040",
            normal: "#606060",
            shadow: "#d4d4d4",
            underground: "#f1f1f1",
            red: "#db4d6d",
            blue: "#58b2dc",
        },
        Theme::Dark => Colors {
            background: "#f3f3f3",
            bg2: "#d4d4d4",
            brand_bg1: "#d4d4d4",
            brand_bg2: "#606060",
            brand_fg1: "#000000",
            brand_fg2: "#f3f3f3",
            rev_bg: "#606060",
            rev_fg: "#f3f3f3",
            rev_shadow: "#606060",
            bold: "#404040",
            normal: "#606060",
            shadow: "#d4d4d4",
            underground: "#f1f1f1",
            red: "#db4d6d",
            blue: "#58b2dc",
        },
    }
}
