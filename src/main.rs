extern crate chrono;
extern crate humansize;
extern crate libc;
extern crate clap;
#[macro_use]

use std::fs;
use std::path::PathBuf;
use std::error::Error;
use std::process;

use clap::Parser;

use chrono::{DateTime, Local};
use humansize::{format_size, DECIMAL};

use libc::{S_IRGRP, S_IROTH, S_IRUSR, S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR};
use std::os::unix::fs::PermissionsExt;


fn parse_permissions(mode: u32) -> String {
    let user = triplet(mode, S_IRUSR, S_IWUSR, S_IXUSR);
    let group = triplet(mode, S_IRGRP, S_IWGRP, S_IXGRP);
    let other = triplet(mode, S_IROTH, S_IWOTH, S_IXOTH);
    [user, group, other].join("")
}

fn triplet(mode: u32, read: u32, write: u32, execute: u32) -> String {
    match(mode & read, mode & write, mode & execute) {
        (0, 0, 0) => "---",
        (_, 0, 0) => "r--",
        (0, _, 0) => "-w-",
        (0, 0, _) => "--x",
        (_, _, 0) => "rw-",
        (_, 0, _) => "r-x",
        (0, _, _) => "-wx",
        (_, _, _) => "rwx"
    }.to_string()
}

#[derive(Parser, Debug)]
struct ListArgs {
    // TODO: fill all this nonsense out
    #[arg()]
    dir: String,
    long: String,
}

impl ListArgs {
    fn new() -> Self {
        // basic info
        let app = App:new("rsls")
            .version("1.0")
            .about("ls-like utility toy written in Rust")
            .author("Joseph Hendrix");
    }
}

fn main() {
    let app = App::new("rsls")
        .version("1.0")
        .about("ls-like utility toy written in Rust")
        .author("Joseph Hendrix");

    // optional args
    let long_option = Arg::with_name("long")
        .short("l")
        .help("Displays metadata for each file")
        .required(false);

    let app = app.arg(long_option);

    let matches = app.get_matches();
}

fn run(dir: &PathBuf) -> Result<(), Box<dyn Error>> {
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
                parse_permissions(mode as u32),
                format_size(size, DECIMAL),
                modified.format("%_d %b %H:%M").to_string(),
                file_name
            );
        }
    }
    Ok(())
}
