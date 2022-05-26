use crate::style::Colors;
use stylist::{css, StyleSource};

#[inline]
pub(super) fn article<'a>(colors: &Colors) -> StyleSource<'a> {
    css!(
        r#"
h1,h2,h3,h4,h5 {
    color: ${bold};
}
a {
    color: ${normal};
    transition: all 0.3s;
}
a::after {
    content: "°";
    position: relative;
    top: -0.15em;
    right: 0.05em;
    color: ${blue};
    transition: all 0.3s;
}
a:hover {
    color: ${bold};
    padding: 0.2em 0;
    background-color: ${underground};
}
a:hover::after {
    color: ${red};
}
ins.html {
    display: inline-block;
    padding-left: 0.5em;
    padding-right: 0.5em;
    padding-bottom: 0;
    margin-left: 0.2em;
    margin-right: 0.2em;
    background-color: ${green_bg};
    color: ${colors_fg};
    border-bottom: none;
    text-decoration: none;
}
ins.html::before {
    color: ${colors_fg};
    content: "+";
    padding-right: 0.4em;
}
del.html {
    display: inline-block;
    padding-left: 0.5em;
    padding-right: 0.5em;
    padding-bottom: 0;
    margin-left: 0.2em;
    margin-right: 0.2em;
    background-color: ${red_bg};
    color: ${colors_fg};
    border-bottom: none;
    text-decoration: none;
}
del.html::before {
    color: ${colors_fg};
    content: "-";
    padding-right: 0.4em;
}
ol,ul {
    margin-left: -2em;
}
blockquote {
    margin: 0 -0.5em 0 -0.8em;
    padding: 0.5em 0.5em;
    border-left: 0.3em double ${shadow};
    font-family: initial !important;
}
blockquote dfn {
    font-weight: normal !important;
}
blockquote p {
    margin: 0;
}
dfn {
    font-style: initial;
    text-decoration: underline dashed ${shadow};
}
em {
    text-decoration: underline double ${shadow};
    color: ${bold};
    text-emphasis: none;
    -webkit-text-emphasis: none;
}
pre {
    margin: 0 -0.5em 0 -0.8em;
    padding: 0.5em 0.5em;
    border-left: 0.3em solid ${shadow};
    display: block;
    background-color: ${underground};
}
pre code {
    border: none;
}
code {
    font-family: "Iosevka Fixed SS10 web";
    border: 0.1em dashed ${shadow};
}
a > code,.phi {
    font-family: 'Phi horizontal';
}
hr {
    background-color: ${underground};
    border: none;
    height: 1px;
}
img {
    max-width: 100%;
}
img.task-marker {
    display: inline;
    width: 0.9em;
    padding-right: 0.4em;
}
img.task-marker + p {
    display: inline;
}
li.task-list {
    list-style-type: none;
}
time {
    color: ${shadow};
}
hr {
    display: inline-block;
    width: 10em;
}
sup a::before,
.fd > a::before {
    content: "[";
    color: ${normal};
}
sup a::after,
.fd > a::after,
sup a:visited::after {
    transition: none;
    content: "]";
    color: ${normal};
    position: static;
}
.fd a {
    padding-right: 0.3em;
}
.fd a + p {
    display: inline;
}
div.footnote-definition p {
    display: inline;
}
"#,
        normal = colors.normal,
        bold = colors.bold,
        shadow = colors.shadow,
        underground = colors.underground,
        red = colors.red,
        blue = colors.blue,
        red_bg = colors.red_bg,
        green_bg = colors.green_bg,
        colors_fg = colors.colors_fg,
    )
}
