extern crate rand;
extern crate walkdir;

use walkdir::WalkDir;
use rand::{Rng, thread_rng};
use std::env;
use std::fs::File;
use std::fs;
use std::io::prelude::*;
use std::io::Error;
use std::cmp;

fn main() {
    let mut recursive: bool = false;
    let mut file: String = String::from("");

    for argument in env::args().skip(1) {
        if argument.to_lowercase() == "--help" || argument.to_lowercase() == "-help" {
            help();
            return;
        } else if argument.to_lowercase() == "--v" || argument.to_lowercase() == "-v" ||
           argument.to_lowercase() == "-version" ||
           argument.to_lowercase() == "--version" {
            version();
            return;
        } else if argument.to_lowercase() == "-r" {
            recursive = true;
        } else {
            file = argument;
            break;
        }
    }
    if file == "" {
        help();
        return;
    }
    file = get_path(&file);

    if recursive {
        for entry in WalkDir::new(file) {
            let entry = entry.unwrap();
            trumpify(entry.path().display().to_string());
        }
    } else {
        trumpify(file)
    }
}

fn get_path2(file: &str) -> Result<String, Error> {
    let path = try!(fs::canonicalize(file));
    Ok(path.display().to_string())
}

fn get_path(file: &str) -> String {
    get_path2(file).unwrap_or(file.to_owned())
}

fn read_file(file: &str) -> Result<String, Error> {
    let mut file = try!(File::open(file));
    let mut contents = String::new();
    try!(file.read_to_string(&mut contents));
    Ok(contents)
}

fn write_file(file: &str, content: String) -> Result<(), Error> {
    let mut f = try!(File::create(file));
    try!(f.write_all(content.as_bytes()));

    try!(f.sync_data());
    Ok(())
}

fn version() {
    println!("\x1b[32mVersion:");
    println!("\x1b[0m    trump-0.1.0");
    println!("\x1b[32mWebsite:");
    println!("\x1b[0m    https://github.com/x13machine/trump/");
    println!("\x1b[32mCreator:");
    println!("\x1b[0m    <Googolplexking/x13machine> https://googolplex.ninja/");
}
fn help() {
    println!("\x1b[32mHelp:");
    println!("\x1b[0m    trump --help");
    println!("\x1b[32mVersion:");
    println!("\x1b[0m    trump -v");
    println!("\x1b[32mFiles:");
    println!("\x1b[0m    trump <File>");
    println!("\x1b[32mDirectories:");
    println!("\x1b[0m    trump -r <Directory>");
}

fn trumpify(file: String) {
    let dirs: Vec<&str> = file.split('/').collect::<Vec<&str>>();
    let mut name: String = String::from(dirs[dirs.len() - 1]);
    name = format!("{}{}",
                   &name[0..1].to_uppercase(),
                   &name[1..name.len()].to_lowercase());
    // Ansi Color Codes!!!
    println!("\x1b[32mMaking Great Again:\x1b[0m {}", file);

    let text = format!("Make {} Great Again!", name);

    let mut content = match read_file(&file) {
        Ok(content) => content,
        Err(_) => return,
    };

    let loops = cmp::max(content.len() / text.len() / 2, 1);
    let mut rng = thread_rng();
    for _ in 0..loops {
        let place: usize = rng.gen_range(0, content.len() as u32) as usize;
        let part1: String = String::from(&content[0..place]);
        let part2: String =
            String::from(&content[cmp::min(place + text.len(), content.len())..content.len()]);
        content = format!("{}{}{}", part1, text, part2);
    }
    let _ = write_file(&file, content);
}
