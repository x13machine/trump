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
    let mut i: i32 = -1;

    for argument in env::args() {
        i += 1;
        if i == 0 {
            continue;
        }

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
    return Ok(path.display().to_string());
}

fn get_path(file: &str) -> String {
    return get_path2(file).unwrap_or(file.to_string());
}

fn read_file2(file: &str) -> Result<String, Error> {
    let mut file = try!(File::open(file));
    let mut contents: Vec<u8> = Vec::new();
    try!(file.read_to_end(&mut contents));
    let ret = String::from_utf8(contents).unwrap_or("".to_string());
    return Ok(ret);
}


fn read_file(file: &str) -> String {
    return read_file2(file).unwrap_or("".to_string());
}

fn write_file2(file: &str, content: String) -> Result<bool, Error> {
    let mut f = try!(File::create(file));
    try!(f.write_all(content.as_bytes()));

    try!(f.sync_data());
    return Ok(false);
}

fn write_file(file: &str, content: String) {
    write_file2(file, content).unwrap_or(false);
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
    let dirs: Vec<&str> = file.split("/").collect::<Vec<&str>>();
    let mut name: String = String::from(dirs[dirs.len() - 1]);
    name = format!("{}{}",
                   &name[0..1].to_uppercase(),
                   &name[1..name.len()].to_lowercase());
    // Ansi Color Codes!!!
    println!("\x1b[32mMaking Great Again:\x1b[0m {}", file);

    let text = format!("Make {} Great Again!", name);

    let mut content: String = read_file(&file);
    if content == "" {
        return;
    }

    let loops = cmp::max(content.len() / text.len() / 2, 1);
    let mut rng = thread_rng();
    for i in 0..loops {
        let place: usize = rng.gen_range(0, content.len() as u32) as usize;
        let part1: String = String::from(&content[0..place]);
        let part2: String =
            String::from(&content[cmp::min(place + text.len(), content.len())..content.len()]);
        content = format!("{}{}{}", part1, text, part2);
    }
    write_file(&file, content);
}
