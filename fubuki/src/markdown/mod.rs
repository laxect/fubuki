mod custom_tag;

use custom_tag::HTag;
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag};
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
                log::error!("stack error");
            } else if let Some(node) = spine.last_mut() {
                node.add_child($child.into());
            }
        }};
    }

    for ev in Parser::new_ext(src, Options::all()) {
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
                depth -= 1;
                if depth > 0 {
                    add_child!(top);
                } else {
                    spine.push(top);
                }
            }
            Event::Text(text) => add_child!(VText::new(text.into_string())),
            Event::Code(text) => {
                let mut code = VTag::new("code");
                code.add_attribute("class", "inline-code");
                code.add_child(VText::new(text.to_string()).into());
                add_child!(code);
            }
            Event::SoftBreak => add_child!(VText::new("\n")),
            Event::HardBreak => add_child!(VTag::new("br")),
            Event::TaskListMarker(done) => {
                if let Some(back) = spine.last_mut() {
                    back.add_attribute("class", "task-list");
                }
                let mut task_marker = VTag::new("img");
                task_marker.add_attribute("class", "task-marker");
                let marker = if done {
                    "icon/check.svg"
                } else {
                    "icon/square.svg"
                };
                task_marker.add_attribute("src", marker);
                add_child!(task_marker);
            }
            Event::Html(html) => {
                let tags = custom_tag::html_parse(html.as_ref());
                for tag in tags.into_iter() {
                    match tag {
                        HTag::Left(t_name) => {
                            depth += 1;
                            let mut v_tag = VTag::new(t_name);
                            v_tag.add_attribute("class", "html");
                            spine.push(v_tag);
                        }
                        HTag::Right(_) => {
                            depth -= 1;
                            if depth > 0 {
                                if let Some(node) = spine.pop() {
                                    add_child!(node);
                                }
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
                v_tag.add_attribute("class", "fr");
                v_tag.add_attribute("id", fr);
                let mut inner = VTag::new("a");
                inner.add_attribute("href", fd);
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
    match t {
        Tag::Paragraph => VTag::new("p"),
        Tag::Heading(n, ..) => VTag::new(format!("h{}", n)),
        Tag::BlockQuote => VTag::new("blockquote"),
        Tag::CodeBlock(lang) => {
            let mut el = VTag::new("code");
            let class = match lang {
                CodeBlockKind::Indented => "".to_owned(),
                CodeBlockKind::Fenced(lang) => lang.into_string(),
            };
            el.add_attribute("class", class);
            el
        }
        Tag::List(None) => VTag::new("ul"),
        Tag::List(Some(1)) => VTag::new("ol"),
        Tag::List(Some(start)) => {
            let mut el = VTag::new("ol");
            el.add_attribute("start", start.to_string());
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
            el.add_attribute("class", "font-weight-bold");
            el
        }
        Tag::Link(_type, href, title) => {
            let mut el = VTag::new("a");
            el.add_attribute("href", href.into_string());
            if title.len() != 0 {
                el.add_attribute("title", title.into_string());
            }
            el
        }
        Tag::Image(_type, src, title) => {
            let mut el = VTag::new("img");
            el.add_attribute("src", src.into_string());
            if title.len() != 0 {
                el.add_attribute("title", title.into_string());
            }
            el
        }
        Tag::FootnoteDefinition(fnn) => {
            let fr = format!("#r:fr:{}", fnn);
            let fd = format!("r:fd:{}", fnn);
            // link to defin
            let mut el = VTag::new("div");
            el.add_attribute("class", "fd");
            el.add_attribute("id", fd);
            // link back
            let mut inner = VTag::new("a");
            inner.add_attribute("href", fr);
            inner.add_child(VText::new(fnn.into_string()).into());
            el.add_child(inner.into());
            el
        }
        Tag::Strikethrough => VTag::new("del"),
    }
}
