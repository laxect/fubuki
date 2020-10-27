mod atom;
mod date;
mod front_matter;

use crate::atom::Post;
use serde::{Deserialize, Serialize};

use std::{
    env, fs,
    io::{self, prelude::*},
    path,
};

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
    if let Some((front_matter, content)) = front_matter::parse_front_matter(contents) {
        let post = front_matter.into_post(entry.file_name().into_string().unwrap().replace(".md", ""));
        Ok(Post {
            front_matter: post,
            content,
        })
    } else {
        Err(io::Error::from(io::ErrorKind::InvalidData))
    }
}

#[derive(Deserialize, Serialize)]
struct PubSubForm {
    #[serde(rename = "hub.mode")]
    pub mode: &'static str,
    #[serde(rename = "hub.url")]
    pub url: &'static str,
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
    posts.sort_by(|a, b| b.front_matter.url.cmp(&a.front_matter.url));
    // get atom xml
    let feed = [dist.clone(), "/atom.xml".into()].concat();
    println!("## write atom xml result to {}", feed);
    let mut atom_output = fs::File::create(feed)?;
    let atom_feed = atom::gather_posts(posts.clone());
    atom_output.write_all(atom_feed.to_string().as_bytes())?;
    // get json
    let json = [dist, "/posts.yml".into()].concat();
    println!("## write post yaml result to {}", json);
    let mut json_output = fs::File::create(json)?;
    // this place will always success, so just unwrap
    let fms: Vec<fubuki_types::Post> = posts
        .into_iter()
        .map(|x| x.front_matter)
        .map(|mut fm| {
            fm.remove_time();
            fm
        })
        .collect();
    json_output.write_all(serde_yaml::to_string(&fms).unwrap().as_bytes())
}
