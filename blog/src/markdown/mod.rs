mod custom_tag;

use custom_tag::HTag;
use pulldown_cmark::{Event, Options, Parser, Tag};
use yew::{
    html,
    virtual_dom::{VNode, VTag, VText},
    Html,
};

pub fn render_markdown(src: &str) -> Html {
    let mut depth: u32 = 0; // virtal stack
    let mut spine = Vec::new();

    macro_rules! add_child {
        ($child:expr) => {{
            if depth < 1 {
                log::warn!("stack error");
            } else if let Some(node) = spine.last_mut() {
                node.add_child($child.into());
            }
        }};
    }

    for ev in Parser::new_ext(src, Options::all()) {
        log::info!("e - {:?}", &ev);
        match ev {
            Event::Start(tag) => {
                depth += 1;
                spine.push(make_tag(tag));
            }
            Event::End(tag) => {
                // there should be at least one tag in spines
                // so just unwrap
                let mut top = spine.pop().unwrap();
                if let Tag::CodeBlock(_) = tag {
                    let mut pre = VTag::new("pre");
                    pre.add_child(top.into());
                    top = pre;
                }
                if depth > 1 {
                    add_child!(top);
                } else {
                    spine.push(top);
                }
                depth -= 1;
            }
            Event::Text(text) => add_child!(VText::new(text.to_string())),
            Event::Code(text) => {
                let mut code = VTag::new("code");
                code.add_class("inline-code");
                code.add_child(VText::new(text.to_string()).into());
                add_child!(code);
            }
            Event::SoftBreak => add_child!(VText::new("\n".to_string())),
            Event::HardBreak => add_child!(VTag::new("br")),
            Event::TaskListMarker(done) => {
                if let Some(back) = spine.last_mut() {
                    back.add_class("task-list");
                }
                let mut task_marker = VTag::new("img");
                task_marker.add_class("task-marker");
                let marker = if done {
                    "icon/check.svg"
                } else {
                    "icon/square.svg"
                };
                task_marker.add_attribute("src", &marker);
                add_child!(task_marker);
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
                            if let Some(node) = spine.pop() {
                                add_child!(node);
                            }
                        }
                        HTag::Text(inner) => {
                            let v_text = VText::new(inner);
                            add_child!(v_text);
                        }
                    };
                }
            }
            Event::FootnoteReference(fnn) => {
                let fr = format!("r:fr:{}", fnn); // self
                let fd = format!("#r:fd:{}", fnn);
                // link to defin
                let mut v_tag = VTag::new("sup");
                v_tag.add_class("fr");
                v_tag.add_attribute("id", &fr);
                let mut inner = VTag::new("a");
                inner.add_attribute("href", &fd);
                inner.add_child(VText::new(fnn.to_string()).into());
                v_tag.add_child(inner.into());
                add_child!(v_tag);
            }
            Event::Rule => {
                let hr_tag = VTag::new("hr");
                if depth > 1 {
                    add_child!(hr_tag);
                } else {
                    spine.push(hr_tag);
                }
            }
        }
    }

    if spine.len() == 1 {
        VNode::VTag(Box::new(spine.pop().unwrap()))
    } else {
        html! {
            <section>{ for spine.into_iter() }</section>
        }
    }
}

fn make_tag(t: Tag) -> VTag {
    log::info!("t - {:?}", &t);
    match t {
        Tag::Paragraph => VTag::new("p"),
        Tag::Heading(n) => VTag::new(format!("h{}", n)),
        Tag::BlockQuote => VTag::new("blockquote"),
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
        Tag::Table(_) => VTag::new("table"),
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
        Tag::FootnoteDefinition(ref fnn) => {
            let fr = format!("#r:fr:{}", fnn);
            let fd = format!("r:fd:{}", fnn);
            // link to defin
            let mut el = VTag::new("div");
            el.add_class("fd");
            el.add_attribute("id", &fd);
            // link back
            let mut inner = VTag::new("a");
            inner.add_attribute("href", &fr);
            inner.add_child(VText::new(fnn.to_string()).into());
            el.add_child(inner.into());
            el
        }
        Tag::Strikethrough => VTag::new("del"),
    }
}
