mod args;

use args::RsArgs;

extern crate chrono;
extern crate clap;
extern crate humansize;
extern crate libc;
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

fn parse_permissions(mode: u16) -> String {
    let user = triplet(mode, S_IRUSR, S_IWUSR, S_IXUSR);
    let group = triplet(mode, S_IRGRP, S_IWGRP, S_IXGRP);
    let other = triplet(mode, S_IROTH, S_IWOTH, S_IXOTH);
    [user, group, other].join("")
}

fn triplet(mode: u16, read: u16, write: u16, execute: u16) -> String {
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
    let args: RsArgs = RsArgs::parse();

    //  debugging moment
    println!("{}", &args.dir.display());

    if args.long {
        if let Err(ref e) = run_long(&args.dir) {
            println!("{}", e);
            process::exit(1);
        }
    } else {
        if let Err(ref e) = run(&args.dir) {
            println!("{}", e);
            process::exit(1);
        }
    }
}

fn run(dir: &PathBuf) -> Result<(), Box<dyn Error>> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let file_name = entry
                .file_name()
                .into_string()
                .or_else(|f| Err(format!("Invalid entry: {:?}", f)))?;
            println!("{}", file_name);
        }
    }
    Ok(())
}

fn run_long(dir: &PathBuf) -> Result<(), Box<dyn Error>> {
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

            //println!("{}", file_name);
            println!(
                "{} {:>5} {} {}",
                parse_permissions(mode as u16),
                format_size(size, DECIMAL),
                modified.format("%_d %b %H:%M").to_string(),
                file_name
            );
        }
    }
    Ok(())
}
