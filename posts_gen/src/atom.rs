use atom_syndication::{Feed, Person};

pub fn gen_atom_feed() -> String {
    let mut feed = Feed::default();
    // person
    let mut me = Person::default();
    me.set_name("gyara");
    me.set_email("me@gyara.moe".to_string());
    me.set_uri("https://blog.gyara.moe".to_string());
    feed.set_authors(vec![me]);
    feed.set_title("Gyara Studio");
    feed.set_id("https://blog.gyara.moe");
    feed.to_string()
}