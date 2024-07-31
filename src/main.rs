mod args;

use args::RsArgs;

extern crate chrono;
extern crate clap;
extern crate humansize;
extern crate libc;
extern crate termsize;
#[macro_use]
use std::fs;
use std::error::Error;
use std::path::PathBuf;
use std::process;

use clap::Parser;

use chrono::{DateTime, Local};
use humansize::{format_size, DECIMAL};

use libc::{S_IRGRP, S_IROTH, S_IRUSR, S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR};
use std::os::unix::fs::PermissionsExt;

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
    let args: RsArgs = RsArgs::parse();

    //  debugging moment
    // println!("{}", &args.dir.display());
    let termcols = termsize::get().unwrap().cols;
    println!("{}", termcols);

    if args.all {
        include_hidden = true;
    }

    if args.long {
        if let Err(ref e) = run_long(include_hidden, &args.dir) {
            println!("{}", e);
            process::exit(1);
        }
    } else {
        if let Err(ref e) = run(include_hidden, &args.dir) {
            println!("{}", e);
            process::exit(1);
        }
    }
}

// TODO: We might wanna push each file_name to a mut vec for
// alphabetization and formatting

fn run(include_hidden: bool, dir: &PathBuf) -> Result<(), Box<dyn Error>> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let file_name = entry
                .file_name()
                .into_string()
                .or_else(|f| Err(format!("Invalid entry: {:?}", f)))?;
            if include_hidden == false {
                // skip hidden files
                if file_name.chars().nth(0) != Some('.') {
                    println!("{}", file_name);
                }
            } else {
                // include hidden files
                println!("{}", file_name);
            }
        }
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
