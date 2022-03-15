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
    pub red_bg: &'static str,
    pub blue_bg: &'static str,
    pub green_bg: &'static str,
    pub colors_fg: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Theme {
    Light,
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
            // usubeni
            red_bg: "#e87a90",
            blue_bg: "#58b2dc",
            // nae
            green_bg: "#86C166",
            colors_fg: "#f3f3f3",
        },
    }
}

pub struct Layout {
    pub nav_bar: f32,
    pub glitch: f32,
    pub footer_top: f32,
    pub footer_main: f32,
    pub footer_bottom: f32,
}

/// All in /0.1 rem
/// 28 is 2.8
const LAYOUT: Layout = Layout {
    // compute value.
    nav_bar: 2.8,
    // Save some space for glitch.
    glitch: 0.4,
    footer_top: 4.0,
    footer_main: 6.0,
    footer_bottom: 2.0,
};

impl Layout {
    pub fn footer() -> f32 {
        LAYOUT.footer_bottom + LAYOUT.footer_main + LAYOUT.footer_top
    }

    pub fn navbar() -> f32 {
        LAYOUT.nav_bar + LAYOUT.glitch
    }

    pub fn layout() -> &'static Layout {
        &LAYOUT
    }
}
