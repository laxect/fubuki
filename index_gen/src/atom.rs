use crate::date;
use atom_syndication::{CategoryBuilder, Content, Entry, Feed, FixedDateTime, Generator, Link, Person, Text};

#[derive(Clone)]
pub struct Post {
    pub front_matter: fubuki_types::Post,
    pub content: String,
}

#[inline(always)]
fn right() -> Text {
    Text::plain("© 2016 - 2021 gyara")
}

impl Post {
    fn id(&self) -> String {
        format!("https://blog.gyara.moe/post/{}/", self.front_matter.url)
    }

    pub fn updated(&self) -> String {
        date::from_jp_to_iso(self.front_matter.date.clone()).unwrap()
    }
}

impl From<Post> for Entry {
    fn from(post: Post) -> Entry {
        let mut entry = Entry::default();
        entry.set_title(post.front_matter.title.clone());
        entry.set_id(post.id());
        // updated
        let updated = post.updated();
        entry.set_updated(FixedDateTime::parse_from_rfc3339(&updated).unwrap());
        entry.set_published(FixedDateTime::parse_from_rfc3339(&updated).unwrap());
        entry.set_authors(vec![get_me()]);
        // link
        let mut link = Link::default();
        link.set_href(post.id());
        link.set_rel("alternate");
        entry.set_links(vec![link]);
        entry.set_rights(right());
        entry.set_summary(Text::plain(post.front_matter.summary));
        // category
        let mut category = CategoryBuilder::default();
        category.label(Some(post.front_matter.category.clone()));
        category.term(post.front_matter.category);
        entry.set_categories(vec![category.build()]);
        // content
        let mut content = Content::default();
        let mut html = String::new();
        let parser = pulldown_cmark::Parser::new(&post.content);
        pulldown_cmark::html::push_html(&mut html, parser);
        let escaped = htmlescape::encode_minimal(&html);
        content.set_value(escaped);
        content.set_content_type("html".to_string());
        entry.set_content(content);
        entry
    }
}

fn get_me() -> Person {
    let mut me = Person::default();
    me.set_name("gyara");
    me.set_email("me@gyara.moe".to_string());
    me.set_uri("https://blog.gyara.moe/".to_string());
    me
}

fn gen_atom_feed() -> Feed {
    let mut feed = Feed::default();
    // generator
    let mut generator = Generator::default();
    generator.set_value("fubuki::posts_gen");
    generator.set_uri("https://github.com/laxect/fubuki/tree/roze/index_gen".to_string());
    generator.set_version(env!("CARGO_PKG_VERSION").to_string());
    // website link
    let mut link = Link::default();
    link.set_href("https://blog.gyara.moe/atom.xml");
    link.set_rel("self".to_string());
    // pubsubhubbub
    let mut pubsubhubbub = Link::default();
    pubsubhubbub.set_href("https://pubsubhubbub.appspot.com/");
    pubsubhubbub.set_rel("hub".to_string());
    // feed
    feed.set_authors(vec![get_me()]);
    feed.set_title("島風造船所");
    feed.set_id("https://blog.gyara.moe/");
    feed.set_generator(generator);
    feed.set_links(vec![link, pubsubhubbub]);
    feed.set_rights(right());
    feed.set_subtitle(Text::plain("島風造船所"));
    feed
}

pub fn gather_posts(posts: Vec<Post>) -> Feed {
    let mut feed = gen_atom_feed();
    let entrys: Vec<Entry> = posts.into_iter().map(std::convert::Into::into).collect();
    if let Some(entry) = entrys.first() {
        feed.set_updated(*entry.updated());
    }
    feed.set_entries(entrys);
    feed
}
