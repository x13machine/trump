extern crate rand;
extern crate walkdir;

use walkdir::WalkDir;
use rand::{Rng, thread_rng};
use std::env;
use std::fs::{self, OpenOptions};
use std::io::prelude::*;
use std::io::{Error, SeekFrom};
use std::cmp;
use std::path::Path;

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
            trumpify_file(entry.path());
        }
    } else {
        trumpify_file(file)
    }
}

fn get_path2(file: &str) -> Result<String, Error> {
    let path = try!(fs::canonicalize(file));
    Ok(path.display().to_string())
}

fn get_path(file: &str) -> String {
    get_path2(file).unwrap_or(file.to_owned())
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

fn trumpify_file<P: AsRef<Path>>(path: P) {
    let path = path.as_ref();

    let name = match path.file_name() {
        Some(name) => name.to_string_lossy(),
        None => return,
    };
    let capitalized_name = format!("{}{}",
                                   &name[0..1].to_uppercase(),
                                   &name[1..name.len()].to_lowercase());
    // Ansi Color Codes!!!
    println!("\x1b[32mMaking Great Again:\x1b[0m {:?}", path);

    let text = format!("Make {} Great Again!", capitalized_name);

    let mut file = match OpenOptions::new().read(true).write(true).open(path) {
        Ok(file) => file,
        Err(_) => return,
    };

    let mut data = Vec::new();

    if let Err(_) = file.read_to_end(&mut data) {
        return;
    }

    if data.is_empty() {
        return;
    }

    trumpify_bytes(&mut data, text.as_bytes());

    let _ = file.seek(SeekFrom::Start(0));
    let _ = file.write_all(&data);
}

fn trumpify_bytes(data: &mut [u8], replace_with: &[u8]) {
    let replace_len = replace_with.len();
    let loops = cmp::max(data.len() / replace_len / 2, 1);
    let mut rng = thread_rng();

    for _ in 0..loops {
        let offset = rng.gen_range(0, data.len());
        data[offset..offset + replace_len].clone_from_slice(replace_with);
    }
}
