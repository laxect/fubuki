mod front_matter;

use std::env;
use std::fs;
use std::io::{self, prelude::*};
use std::path;

/// return command option
/// return (dist, orig)
fn read_option() -> (String, String) {
    let mut args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("invalid argv");
    }
    (
        args.pop().unwrap_or_else(|| "dist".into()),
        args.pop().unwrap_or_else(|| "post".into()),
    )
}

fn file_handle(entry: &fs::DirEntry) -> io::Result<String> {
    println!("::  {}", entry.path().to_str().unwrap());
    let mut file = fs::File::open(entry.path())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(front_matter::parse_front_matter(contents))
}

/// entry
pub fn read_files() -> io::Result<()> {
    let (dist, orig) = read_option();
    println!("{} -> {}", orig, dist);
    let from = path::Path::new(&orig);
    assert!(from.exists(), "origin not exist!");
    assert!(from.is_dir(), "origin is not a directory!");
    for file in fs::read_dir(from)? {
        file_handle(&file?)?;
    }
    Ok(())
}
