mod front_matter;

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

fn file_handle(entry: &fs::DirEntry) -> io::Result<FrontMatter> {
    println!("::  {}", entry.path().to_str().unwrap());
    let mut file = fs::File::open(entry.path())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    if let Some(mut fm) = front_matter::parse_front_matter(contents) {
        fm.fill_url(entry.file_name().into_string().unwrap().replace(".md", ""));
        Ok(fm)
    } else {
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
}

/// entry
pub fn read_files() -> io::Result<()> {
    let (mut dist, orig) = read_option();
    println!("{} -> {}", orig, dist);
    let from = path::Path::new(&orig);
    assert!(from.exists(), "origin not exist!");
    assert!(from.is_dir(), "origin is not a directory!");
    let mut fms = Vec::new();
    for file in fs::read_dir(from)? {
        if let Ok(fm) = file_handle(&file?) {
            fms.push(fm);
        }
    }
    // fms sort
    fms.sort_by(|a, b| b.get_url().cmp(a.get_url()));
    dist.push_str("/posts.json");
    println!("## write result to {}", dist);
    let mut output = fs::File::create(dist)?;
    // json
    output.write_all(serde_json::to_string(&fms).unwrap().as_bytes())
}
