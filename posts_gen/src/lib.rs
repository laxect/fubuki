mod atom;
mod date;
mod front_matter;

use crate::atom::Post;
use crate::front_matter::FrontMatter;

use std::env;
use std::fs;
use std::io::{self, prelude::*};
use std::path;

/// return command option
/// return (dist, orig)
fn read_option() -> (String, String) {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    (
        args.pop().unwrap_or_else(|| "dist".into()),
        args.pop().unwrap_or_else(|| "post".into()),
    )
}

fn file_handle(entry: &fs::DirEntry) -> io::Result<Post> {
    if entry.path().is_dir() {
        return Err(io::Error::from(io::ErrorKind::Other));
    }
    println!("::  {}", entry.path().to_str().unwrap());
    let mut file = fs::File::open(entry.path())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    if let Some((mut front_matter, content)) = front_matter::parse_front_matter(contents) {
        front_matter.fill_url(entry.file_name().into_string().unwrap().replace(".md", ""));
        Ok(Post {
            front_matter,
            content,
        })
    } else {
        Err(io::Error::from(io::ErrorKind::InvalidData))
    }
}

/// update pubsubhubbub
fn update_pubsubhubbub() -> Result<(), Box<std::error::Error>> {
    let mut body = std::collections::HashMap::new();
    body.insert("hub.mode", "publish");
    body.insert("hub.url", "https://blog.gyara.moe/atom.xml");
    let client = reqwest::Client::new();
    client
        .post("https://pubsubhubbub.appspot.com/")
        .form(&body)
        .send()?;
    Ok(())
}

/// entry
pub fn read_files() -> io::Result<()> {
    let (dist, orig) = read_option();
    println!("{} -> {}", orig, dist);
    let from = path::Path::new(&orig);
    assert!(from.exists(), "origin not exist!");
    assert!(from.is_dir(), "origin is not a directory!");
    let mut posts = Vec::new();
    for file in fs::read_dir(from)? {
        if let Ok(fm) = file_handle(&file?) {
            posts.push(fm);
        }
    }
    // fms sort
    posts.sort_by(|a, b| b.front_matter.get_url().cmp(a.front_matter.get_url()));
    // get atom xml
    let feed = [dist.clone(), "/atom.xml".into()].concat();
    println!("## write atom xml result to {}", feed);
    let mut atom_output = fs::File::create(feed)?;
    let atom_feed = atom::gather_posts(posts.clone());
    atom_output.write_all(atom_feed.to_string().as_bytes())?;
    match update_pubsubhubbub() {
        Err(e) => {
            println!("## publish failed {}", e);
            return Err(io::Error::from(io::ErrorKind::Interrupted));
        }
        Ok(r) => {
            println!("    {:?}", r);
        }
    }
    // get json
    let json = [dist, "/post.json".into()].concat();
    println!("## write post json result to {}", json);
    let mut json_output = fs::File::create(json)?;
    // this place will always success, so just unwrap
    let fms: Vec<FrontMatter> = posts.into_iter().map(|x| x.front_matter).collect();
    json_output.write_all(serde_json::to_string(&fms).unwrap().as_bytes())
}
