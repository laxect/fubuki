/// Original author of this code is [Nathan Ringo](https://github.com/remexre)
/// Source: https://github.com/acmumn/mentoring/blob/master/web-client/src/view/markdown.rs
use pulldown_cmark::{Alignment, Event, Options, Parser, Tag};
use yew::virtual_dom::{VNode, VTag, VText};
use yew::{html, Component, Html};

pub fn render_markdown<COMP>(src: &str) -> Html<COMP>
where
    COMP: Component,
{
    let mut elems = vec![];
    let mut spine = vec![];

    macro_rules! add_child {
        ($child:expr) => {{
            let l = spine.len();
            assert_ne!(l, 0);
            spine[l - 1].add_child($child);
        }};
    }

    for ev in Parser::new_ext(src, Options::all()) {
        match ev {
            Event::Start(tag) => {
                spine.push(make_tag(tag));
            }
            Event::End(tag) => {
                let l = spine.len();
                assert!(l >= 1);
                let mut top = spine.pop().unwrap();
                if let Tag::CodeBlock(_) = tag {
                    let mut pre = VTag::new("pre");
                    pre.add_child(top.into());
                    top = pre;
                } else if let Tag::Table(aligns) = tag {
                    for r in top.childs.iter_mut() {
                        if let VNode::VTag(ref mut vtag) = r {
                            for (i, c) in vtag.childs.iter_mut().enumerate() {
                                if let VNode::VTag(ref mut vtag) = c {
                                    match aligns[i] {
                                        Alignment::None => {}
                                        Alignment::Left => vtag.add_class("text-left"),
                                        Alignment::Center => vtag.add_class("text-center"),
                                        Alignment::Right => vtag.add_class("text-right"),
                                    }
                                }
                            }
                        }
                    }
                } else if let Tag::TableHead = tag {
                    for c in top.childs.iter_mut() {
                        if let VNode::VTag(ref mut vtag) = c {
                            vtag.add_attribute("scope", &"col");
                        }
                    }
                }
                if l == 1 {
                    elems.push(top);
                } else {
                    spine[l - 2].add_child(top.into());
                }
            }
            Event::Text(text) => add_child!(VText::new(text.to_string()).into()),
            Event::SoftBreak => add_child!(VText::new("\n".to_string()).into()),
            Event::HardBreak => add_child!(VTag::new("br").into()),
            _ => println!("Unknown event: {:#?}", ev),
        }
    }

    if elems.len() == 1 {
        VNode::VTag(elems.pop().unwrap())
    } else {
        html! {
            <div>{ for elems.into_iter() }</div>
        }
    }
}

fn make_tag<COMP>(t: Tag) -> VTag<COMP>
where
    COMP: Component,
{
    match t {
        Tag::Paragraph => VTag::new("p"),
        Tag::Rule => VTag::new("hr"),
        Tag::Header(n) => {
            assert!(n > 0);
            assert!(n < 7);
            VTag::new(format!("h{}", n))
        }
        Tag::BlockQuote => {
            let mut el = VTag::new("blockquote");
            el.add_class("blockquote");
            el
        }
        Tag::CodeBlock(lang) => {
            let mut el = VTag::new("code");
            // Different color schemes may be used for different code blocks,
            // but a different library (likely js based at the moment) would be necessary to actually provide the
            // highlighting support by locating the language classes and applying dom transforms
            // on their contents.
            el.add_class(lang.as_ref());
            el
        }
        Tag::List(None) => VTag::new("ul"),
        Tag::List(Some(1)) => VTag::new("ol"),
        Tag::List(Some(ref start)) => {
            let mut el = VTag::new("ol");
            el.add_attribute("start", start);
            el
        }
        Tag::Item => VTag::new("li"),
        Tag::Table(_) => {
            let mut el = VTag::new("table");
            el.add_class("table");
            el
        }
        Tag::TableHead => VTag::new("th"),
        Tag::TableRow => VTag::new("tr"),
        Tag::TableCell => VTag::new("td"),
        Tag::Emphasis => VTag::new("dfn"),
        Tag::Strong => {
            let mut el = VTag::new("em");
            el.add_class("font-weight-bold");
            el
        }
        Tag::Code => {
            let mut el = VTag::new("code");
            el.add_class("inline-code");
            el
        }
        Tag::Link(ref _type, ref href, ref title) => {
            let mut el = VTag::new("a");
            el.add_attribute("href", href);
            if title.len() != 0 {
                el.add_attribute("title", title);
            }
            el
        }
        Tag::Image(ref _type, ref src, ref title) => {
            let mut el = VTag::new("img");
            el.add_attribute("src", src);
            if title.len() != 0 {
                el.add_attribute("title", title);
            }
            el
        }
        Tag::FootnoteDefinition(ref _id) => VTag::new("span"), // Footnotes are not rendered as anything special
        Tag::HtmlBlock => VTag::new("div"),
        Tag::Strikethrough => VTag::new("del"),
    }
}
