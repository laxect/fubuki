mod custom_tag;

use custom_tag::HTag;
use pulldown_cmark::{Event, Options, Parser, Tag};
use yew::{
    html,
    virtual_dom::{VNode, VTag, VText},
    Component, Html,
};

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
            Event::Code(text) => {
                let mut code = VTag::new("code");
                code.add_class("inline-code");
                code.add_child(VText::new(text.to_string()).into());
                add_child!(code.into())
            }
            Event::SoftBreak => add_child!(VText::new("\n".to_string()).into()),
            Event::HardBreak => add_child!(VTag::new("br").into()),
            Event::TaskListMarker(done) => {
                if let Some(back) = spine.last_mut() {
                    back.add_class("task-list");
                }
                let mut task_marker = VTag::new("img");
                task_marker.add_class("task-marker");
                let marker = if done { "icon/check.svg" } else { "icon/square.svg" };
                task_marker.add_attribute("src" , &marker);
                add_child!(task_marker.into());
            }
            Event::Html(html) => {
                let tags = custom_tag::html_parse(html.as_ref());
                for tag in tags.into_iter() {
                    match tag {
                        HTag::Left(t_name) => {
                            let mut v_tag = VTag::new(t_name);
                            v_tag.add_class("html");
                            spine.push(v_tag);
                        }
                        HTag::Right(_) => {
                            let v_tag = spine.pop().unwrap();
                            add_child!(v_tag.into());
                        }
                        HTag::Text(inner) => {
                            let v_text = VText::new(inner);
                            add_child!(v_text.into());
                        }
                    };
                }
            }
            Event::InlineHtml(html) => {
                let tag = HTag::from_string(html.as_ref());
                match tag {
                    HTag::Left(t_name) => {
                        let mut v_tag = VTag::new(t_name);
                        v_tag.add_class("inline-html");
                        spine.push(v_tag);
                    }
                    HTag::Right(_) => {
                        let v_tag = spine.pop().unwrap();
                        add_child!(v_tag.into());
                    }
                    _ => unreachable!(),
                };
            }
            Event::FootnoteReference(_) => {}
        }
    }

    if elems.len() == 1 {
        VNode::VTag(elems.pop().unwrap())
    } else {
        html! {
            <section>{ for elems.into_iter() }</section>
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
        Tag::FootnoteDefinition(ref _id) => VTag::new("span"),
        Tag::HtmlBlock => {
            let mut el = VTag::new("div");
            el.add_class("html");
            el
        }
        Tag::Strikethrough => VTag::new("del"),
    }
}
