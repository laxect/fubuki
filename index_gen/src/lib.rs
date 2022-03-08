mod atom;
mod date;
mod front_matter;

use crate::atom::Post;
use blake2::Digest;
use std::{
    env, fs,
    io::{Read, Write},
    path,
};

/// return command option
/// return (dist, orig)
fn read_option() -> (String, String) {
    let mut args: Vec<String> = env::args().collect();
    (
        args.pop().unwrap_or_else(|| "dist".into()),
        args.pop().unwrap_or_else(|| "post".into()),
    )
}

fn file_handle(entry: &fs::DirEntry) -> anyhow::Result<Post> {
    if entry.path().is_dir() {
        return Err(anyhow::Error::msg("Not a file."));
    }
    println!("::  {}", entry.path().to_string_lossy());
    let mut file = fs::File::open(entry.path())?;
    let mut contents = String::new();
    let mut hasher = blake2::Blake2b512::new();
    file.read_to_string(&mut contents)?;
    hasher.update(contents.as_bytes());
    let res = hasher.finalize();
    let hash = base64::encode(res);
    if let Ok((front_matter, content)) = front_matter::parse_front_matter(contents) {
        let post = front_matter.into_post(entry.file_name().to_string_lossy().replace(".md", ""), hash);
        return Ok(Post {
            front_matter: post,
            content,
        });
    } else {
        println!("x {} parse failed", entry.file_name().to_string_lossy());
    }
    Err(anyhow::Error::msg("File handle failed"))
}

/// entry
pub fn read_files() -> anyhow::Result<()> {
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
    atom_feed.write_to(&mut atom_output)?;
    // get yaml
    let yaml = [dist, "/posts.yml".into()].concat();
    println!("## write post yaml result to {}", yaml);
    let mut yaml_output = fs::File::create(yaml)?;
    // this place will always success, so just unwrap
    let fms: Vec<fubuki_types::Post> = posts.into_iter().map(|x| x.front_matter).collect();
    yaml_output.write_all(serde_yaml::to_string(&fms).unwrap().as_bytes())?;
    Ok(())
}
