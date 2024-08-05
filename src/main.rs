mod args;

use args::RsArgs;

extern crate chrono;
extern crate clap;
extern crate humansize;
extern crate libc;
extern crate termsize;
use std::cmp;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::process;

use clap::Parser;

use chrono::{DateTime, Local};
use humansize::{format_size, DECIMAL};

use libc::{S_IRGRP, S_IROTH, S_IRUSR, S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR};
use std::os::unix::fs::PermissionsExt;
use std::collections::HashMap;

const RESET: &str = "\x1b[0m";
const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const MAGENTA: &str = "\x1b[35m";

fn parse_permissions(mode: u32) -> String {
    let user = triplet(mode, S_IRUSR as u32, S_IWUSR as u32, S_IXUSR as u32);
    let group = triplet(mode, S_IRGRP as u32, S_IWGRP as u32, S_IXGRP as u32);
    let other = triplet(mode, S_IROTH as u32, S_IWOTH as u32, S_IXOTH as u32);
    [user, group, other].join("")
}

fn triplet(mode: u32, read: u32, write: u32, execute: u32) -> String {
    match (mode & read, mode & write, mode & execute) {
        (0, 0, 0) => "---",
        (_, 0, 0) => "r--",
        (0, _, 0) => "-w-",
        (0, 0, _) => "--x",
        (_, _, 0) => "rw-",
        (_, 0, _) => "r-x",
        (0, _, _) => "-wx",
        (_, _, _) => "rwx",
    }
    .to_string()
}

fn main() {
    let mut include_hidden = false;
    let mut force_columns = false;
    let file_icons = create_icon_hashmap();
    let args: RsArgs = RsArgs::parse();

    if args.all {
        include_hidden = true;
    }

    if args.force_col {
        force_columns = true;
    }

    if args.long {
        if let Err(ref e) = run_long(include_hidden, &args.dir) {
            println!("{}", e);
            process::exit(1);
        }
    } else {
        if let Err(ref e) = run(include_hidden, force_columns, &args.dir) {
            println!("{}", e);
            process::exit(1);
        }
    }
}

// TODO: I don't think I need this
fn append_icon(file_name: String) -> String {
    // check if directory
    let split_name = &file_name.split('/').collect::<Vec<&str>>();
    if split_name.len() == 2 {
        //println!("{BLUE} {file_name}{RESET}");
        let result = String::from("{BLUE} {file_name}{RESET}");
        println!("{result}");
        result
    } else {
        file_name
    }
}

fn create_icon_hashmap() -> HashMap<&'static str, &'static str> {
    let mut file_icons = HashMap::new();
    file_icons.insert(".go", " ");
    file_icons.insert(".mod", " ");
    file_icons.insert(".sh"," ");
    file_icons.insert(".cpp"," ");
    file_icons.insert(".hpp"," ");
    file_icons.insert(".cxx"," ");
    file_icons.insert(".hxx"," ");
    file_icons.insert(".css"," ");
    file_icons.insert(".c"," ");
    file_icons.insert("h"," ");
    file_icons.insert(".cs","󰌛 ");
    file_icons.insert(".png"," ");
    file_icons.insert(".jpg"," ");
    file_icons.insert(".JPG"," ");
    file_icons.insert(".jpeg"," ");
    file_icons.insert(".webp"," ");
    file_icons.insert(".xcf"," ");
    file_icons.insert(".xml","󰗀 ");
    file_icons.insert(".htm"," ");
    file_icons.insert(".html"," ");
    file_icons.insert(".txt"," ");
    file_icons.insert(".mp3"," ");
    file_icons.insert(".m4a"," ");
    file_icons.insert(".ogg"," ");
    file_icons.insert(".flac"," ");
    file_icons.insert(".mp4"," ");
    file_icons.insert(".mkv"," ");
    file_icons.insert(".webm"," ");
    file_icons.insert(".zip","󰿺 ");
    file_icons.insert(".tar","󰛫 ");
    file_icons.insert(".gz","󰛫 ");
    file_icons.insert(".bz2","󰿺 ");
    file_icons.insert(".xz","󰿺 ");
    file_icons.insert(".jar"," ");
    file_icons.insert(".java"," ");
    file_icons.insert(".js"," ");
    file_icons.insert(".json"," ");
    file_icons.insert(".py"," ");
    file_icons.insert(".rs"," ");
    file_icons.insert(".yml"," ");
    file_icons.insert(".yaml"," ");
    file_icons.insert(".toml"," ");
    file_icons.insert(".deb"," ");
    file_icons.insert(".md"," ");
    file_icons.insert(".rb"," ");
    file_icons.insert(".php"," ");
    file_icons.insert(".pl"," ");
    file_icons.insert(".svg","󰜡 ");
    file_icons.insert(".eps"," ");
    file_icons.insert(".ps"," ");
    file_icons.insert(".git"," ");
    file_icons.insert(".zig"," ");
    file_icons.insert(".xbps"," ");
    file_icons.insert(".el"," ");
    file_icons.insert(".vim"," ");
    file_icons.insert(".lua"," ");
    file_icons.insert(".pdf"," ");
    file_icons.insert(".epub","󰂺 ");
    file_icons.insert(".conf"," ");
    file_icons.insert(".iso"," ");
    file_icons.insert(".exe"," ");
    file_icons.insert(".odt","󰷈 ");
    file_icons.insert(".ods","󰱾 ");
    file_icons.insert(".odp","󰈧 ");
    file_icons.insert(".gif","󰵸 ");
    file_icons.insert(".tiff","󰋪 ");
    file_icons.insert(".7z"," ");
    file_icons.insert(".bat"," ");
    file_icons.insert(".app"," ");
    file_icons.insert(".log"," ");
    file_icons.insert(".sql"," ");
    file_icons.insert(".db"," ");

    file_icons
}

fn output_to_term(mut files: Vec<String>, force_col: bool, longest_file_name: usize) {
    files.sort();
    let ncol = termsize::get().unwrap().cols / (4 + longest_file_name as u16);
    let nrow = files.len() as u16 / ncol;
    let total_files = files.len();
    println!("total files: {total_files}    nrows: {nrow}    ncol: {ncol}");

    let mut n = 0;
    for entry in files {
        // TODO print file here, check for column output after
        let split_name = &entry.split('/').collect::<Vec<&str>>();
        if split_name.len() == 2 && split_name[1] == "" {
            print!("{BLUE} {entry}{RESET}");
        } else {
            // TODO: if icon isn't found append two empty space
            print!("{entry}  ");
        }
        if force_col {
            println!();
        } else {
            n += 1;
            if n as u16 >= ncol || entry.len() > longest_file_name {
                println!();
                n = 0;
            } else {
                let padding = " ".repeat(4 + longest_file_name - entry.len());
                print!("{padding}");
            }
        }
    }
}

fn run(include_hidden: bool, force_col: bool, dir: &PathBuf) -> Result<(), Box<dyn Error>> {
    //let mut files = Vec::new();
    if dir.is_dir() {
        let mut files = Vec::new();
        let mut longest_file_name = 0;

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let mut file_name = entry
                .file_name()
                .into_string()
                .or_else(|f| Err(format!("Invalid entry: {:?}", f)))?;
            if entry.metadata()?.is_dir() {
                file_name = file_name + "/";
            }
            if !include_hidden {
                // skip hidden files
                if file_name.chars().nth(0) != Some('.') {
                    // println!("{}", file_name);
                    longest_file_name =
                        cmp::max(longest_file_name as usize, file_name.chars().count());
                    files.push(file_name);
                }
            } else {
                // include hidden files
                // println!("{}", file_name);
                longest_file_name = cmp::max(longest_file_name as usize, file_name.len());
                files.push(file_name);
            }
        }
        output_to_term(files, force_col, longest_file_name);
        println!("");
    }
    Ok(())
}

fn run_long(include_hidden: bool, dir: &PathBuf) -> Result<(), Box<dyn Error>> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let file_name = entry
                .file_name()
                .into_string()
                .or_else(|f| Err(format!("Invalid entry: {:?}", f)))?;
            let metadata = entry.metadata()?;
            let size = metadata.len();
            let modified: DateTime<Local> = DateTime::from(metadata.modified()?);
            let mode = metadata.permissions().mode();

            if include_hidden == false {
                // skip hidden files
                if file_name.chars().nth(0) != Some('.') {
                    println!(
                        "{} {:>5} {} {}",
                        if entry.metadata()?.is_dir() {
                            "d".to_string() + &parse_permissions(mode as u32)
                        } else {
                            "-".to_string() + &parse_permissions(mode as u32)
                        },
                        format_size(size, DECIMAL),
                        modified.format("%_d %b %H:%M").to_string(),
                        file_name
                    );
                }
            } else {
                // include hidden files
                println!(
                    "{} {:>5} {} {}",
                    if entry.metadata()?.is_dir() {
                        "d".to_string() + &parse_permissions(mode as u32)
                    } else {
                        "-".to_string() + &parse_permissions(mode as u32)
                    },
                    format_size(size, DECIMAL),
                    modified.format("%_d %b %H:%M").to_string(),
                    file_name
                );
            }
        }
    }
    Ok(())
}
