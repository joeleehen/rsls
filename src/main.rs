mod args;

use args::RsArgs;

extern crate chrono;
extern crate clap;
extern crate humansize;
extern crate libc;
extern crate termsize;
use std::fs;
use std::error::Error;
use std::path::PathBuf;
use std::process;
use std::cmp;

use clap::Parser;

use chrono::{DateTime, Local};
use humansize::{format_size, DECIMAL};

use libc::{S_IRGRP, S_IROTH, S_IRUSR, S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR};
use std::os::unix::fs::PermissionsExt;

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

fn output_to_term(mut files: Vec<String>, force_col: bool, longest_file_name: usize) {
    files.sort();
    let ncol = termsize::get().unwrap().cols / longest_file_name as u16;

    let mut n = 0;
    for entry in files {
        if force_col {
            let mut split_name = &entry.split('/').collect::<Vec<&str>>();
            if split_name.len() == 2 && split_name[1] == "" {
                println!("{BLUE} {entry}{RESET}");
            } else {
                println!("{entry}");
            }
        } else {
            let mut split_name = &entry.split('/').collect::<Vec<&str>>();
            if split_name.len() == 2 && split_name[1] == "" {
                print!("{BLUE} {entry}    {RESET}")
            } else {
                print!("{entry}    ");
            }
            //print!("{entry}    ");
            n += 1;
            if n >= ncol {
                println!("");
                n = 0;
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
                    longest_file_name = cmp::max(longest_file_name as usize, file_name.chars().count());
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
