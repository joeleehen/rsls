mod args;
mod colors;

use colors::*;
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
use std::collections::HashMap;
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
        if let Err(ref e) = run(include_hidden, force_columns, file_icons, &args.dir) {
            println!("{}", e);
            process::exit(1);
        }
    }
}

fn create_icon_hashmap() -> HashMap<String, &'static str> {
    let mut file_icons = HashMap::new();
    file_icons.insert("go".to_string(), " ");
    file_icons.insert("mod".to_string(), " ");
    file_icons.insert("sh".to_string(), " ");
    file_icons.insert("cpp".to_string(), " ");
    file_icons.insert("hpp".to_string(), " ");
    file_icons.insert("cxx".to_string(), " ");
    file_icons.insert("hxx".to_string(), " ");
    file_icons.insert("css".to_string(), " ");
    file_icons.insert("c".to_string(), " ");
    file_icons.insert("h".to_string(), " ");
    file_icons.insert("cs".to_string(), "󰌛 ");
    file_icons.insert("png".to_string(), " ");
    file_icons.insert("jpg".to_string(), " ");
    file_icons.insert("JPG".to_string(), " ");
    file_icons.insert("jpeg".to_string(), " ");
    file_icons.insert("webp".to_string(), " ");
    file_icons.insert("xcf".to_string(), " ");
    file_icons.insert("xml".to_string(), "󰗀 ");
    file_icons.insert("htm".to_string(), " ");
    file_icons.insert("html".to_string(), " ");
    file_icons.insert("txt".to_string(), " ");
    file_icons.insert("mp3".to_string(), " ");
    file_icons.insert("m4a".to_string(), " ");
    file_icons.insert("ogg".to_string(), " ");
    file_icons.insert("flac".to_string(), " ");
    file_icons.insert("mp4".to_string(), " ");
    file_icons.insert("mkv".to_string(), " ");
    file_icons.insert("webm".to_string(), " ");
    file_icons.insert("zip".to_string(), "󰿺 ");
    file_icons.insert("tar".to_string(), "󰛫 ");
    file_icons.insert("gz".to_string(), "󰛫 ");
    file_icons.insert("bz2".to_string(), "󰿺 ");
    file_icons.insert("xz".to_string(), "󰿺 ");
    file_icons.insert("jar".to_string(), " ");
    file_icons.insert("java".to_string(), " ");
    file_icons.insert("js".to_string(), " ");
    file_icons.insert("json".to_string(), " ");
    file_icons.insert("py".to_string(), " ");
    file_icons.insert("rs".to_string(), " ");
    file_icons.insert("yml".to_string(), " ");
    file_icons.insert("yaml".to_string(), " ");
    file_icons.insert("toml".to_string(), " ");
    file_icons.insert("deb".to_string(), " ");
    file_icons.insert("md".to_string(), " ");
    file_icons.insert("rb".to_string(), " ");
    file_icons.insert("php".to_string(), " ");
    file_icons.insert("pl".to_string(), " ");
    file_icons.insert("svg".to_string(), "󰜡 ");
    file_icons.insert("eps".to_string(), " ");
    file_icons.insert("ps".to_string(), " ");
    file_icons.insert("git".to_string(), " ");
    file_icons.insert("zig".to_string(), " ");
    file_icons.insert("xbps".to_string(), " ");
    file_icons.insert("el".to_string(), " ");
    file_icons.insert("vim".to_string(), " ");
    file_icons.insert("lua".to_string(), " ");
    file_icons.insert("pdf".to_string(), " ");
    file_icons.insert("epub".to_string(), "󰂺 ");
    file_icons.insert("conf".to_string(), " ");
    file_icons.insert("iso".to_string(), " ");
    file_icons.insert("exe".to_string(), " ");
    file_icons.insert("odt".to_string(), "󰷈 ");
    file_icons.insert("ods".to_string(), "󰱾 ");
    file_icons.insert("odp".to_string(), "󰈧 ");
    file_icons.insert("gif".to_string(), "󰵸 ");
    file_icons.insert("tiff".to_string(), "󰋪 ");
    file_icons.insert("7z".to_string(), " ");
    file_icons.insert("bat".to_string(), " ");
    file_icons.insert("app".to_string(), " ");
    file_icons.insert("log".to_string(), " ");
    file_icons.insert("sql".to_string(), " ");
    file_icons.insert("db".to_string(), " ");
    file_icons.insert("csv".to_string(), "󰸦 ");
    file_icons.insert("R".to_string(), " ");
    file_icons.insert("doc".to_string(), "󰧮 ");
    file_icons.insert("docx".to_string(), "󰧮 ");
    file_icons.insert("ipynb".to_string(), " ");
    file_icons.insert("xls".to_string(), "󰱾 ");
    file_icons.insert("xlsx".to_string(), "󰱾 ");

    file_icons
}

fn output_to_term(
    mut files: Vec<String>,
    force_col: bool,
    longest_file_name: usize,
    file_icons: HashMap<String, &str>,
) {
    files.sort();
    let ncol = termsize::get().unwrap().cols / (4 + longest_file_name as u16);

    let mut n = 0;
    for entry in files {
        let split_name = &entry.split('/').collect::<Vec<&str>>();

        // printing directories
        if split_name.len() == 2 && split_name[1] == "" {
            print!("{BLUE}{entry} {RESET}");

        // printing files
        } else {
            let split_name = &mut entry.split('.').collect::<Vec<&str>>();
            if split_name.len() < 2 || split_name[0] == "" {
                // handle hidden files/ files that don't have an extension
                print!("{entry}");
                if force_col {
                    println!("");
                } else {
                    n += 1;
                    if n as u16 >= ncol || entry.len() > longest_file_name {
                        println!();
                        n = 0;
                    } else {
                        let padding = " ".repeat(6 + longest_file_name - entry.len());
                        print!("{padding}");
                    }
                }
                continue;
            }

            //let extension = split_name.reverse()[0];
            split_name.reverse();
            let extension = split_name[0];

            let mut skipped = false;
            let icon = file_icons.get(extension);
            if icon.is_some() {
                let icon = icon.unwrap().to_string();
                match extension {
                    "go" => print!("{CYAN}{icon}{RESET}"),
                    "sh" => print!("{BRIGHTGREEN}{icon}{RESET}"),
                    "cpp" | "hpp" | "cxx" | "hxx" => print!("{BLUE}{icon}{RESET}"),
                    "css" => print!("{LIGHTBLUE}{icon}{RESET}"),
                    "c" | "h" => print!("{BLUE}{icon}{RESET}"),
                    "cs" => print!("{DARKMAGENTA}{icon}{RESET}"),
                    "png" | "jpg" | "jpeg" | "JPG" | "webp" => {
                        print!("{BRIGHTMAGENTA}{icon}{RESET}")
                    }
                    "gif" => print!("{MAGENTA}{icon}{RESET}"),
                    "xcf" => print!("{PURPLE}{icon}{RESET}"),
                    "xml" => print!("{LIGHTCYAN}{icon}{RESET}"),
                    "htm" | "html" => print!("{ORANGE}{icon}{RESET}"),
                    "txt" | "app" => print!("{WHITE}{icon}{RESET}"),
                    "mp3" | "m4a" | "ogg" | "flac" => print!("{BRIGHTBLUE}{icon}{RESET}"),
                    "mp4" | "mkv" | "webm" => print!("{BRIGHTMAGENTA}{icon}{RESET}"),
                    "zip" | "tar" | "gz" | "bz2" | "xz" | "7z" => {
                        print!("{LIGHTPURPLE}{icon}{RESET}")
                    }
                    "jar" | "java" => print!("{ORANGE}{icon}{RESET}"),
                    "js" => print!("{YELLOW}{icon}{RESET}"),
                    "json" | "tiff" => print!("{BRIGHTYELLOW}{icon}{RESET}"),
                    "py" => print!("{DARKYELLOW}{icon}{RESET}"),
                    "rs" => print!("{DARKGRAY}{icon}{RESET}"),
                    "yml" | "yaml" => print!("{BRIGHTRED}{icon}{RESET}"),
                    "toml" => print!("{DARKORANGE}{icon}{RESET}"),
                    "deb" => print!("{LIGHTRED}{icon}{RESET}"),
                    "md" => print!("{CYAN}{icon}{RESET}"),
                    "rb" => print!("{RED}{icon}{RESET}"),
                    "php" => print!("{BRIGHTBLUE}{icon}{RESET}"),
                    "pl" => print!("{RED}{icon}{RESET}"),
                    "svg" => print!("{LIGHTPURPLE}{icon}{RESET}"),
                    "eps" | "ps" => print!("{ORANGE}{icon}{RESET}"),
                    "git" => print!("{ORANGE}{icon}{RESET}"),
                    "zig" => print!("{DARKORANGE}{icon}{RESET}"),
                    "xbps" => print!("{DARKGREEN}{icon}{RESET}"),
                    "el" => print!("{PURPLE}{icon}{RESET}"),
                    "vim" => print!("{DARKGREEN}{icon}{RESET}"),
                    "lua" | "sql" => print!("{BRIGHTBLUE}{icon}{RESET}"),
                    "pdf" | "db" => print!("{BRIGHTRED}{icon}{RESET}"),
                    "epub" => print!("{CYAN}{icon}{RESET}"),
                    "conf" | "bat" => print!("{DARKGRAY}{icon}{RESET}"),
                    "iso" => print!("{GRAY}{icon}{RESET}"),
                    "exe" => print!("{BRIGHTCYAN}{icon}{RESET}"),
                    "log" => print!("{GRAY}{icon}{RESET}"),
                    "csv" => print!("{CYAN}{icon}{RESET}"),
                    "R" => print!("{BLUE}{icon}{RESET}"),
                    "doc" | "docx" => print!("{LIGHTCYAN}{icon}{RESET}"),
                    "ipynb" => print!("{YELLOW}{icon}{RESET}"),
                    "xls" | "xlsx" => print!("{LIGHTGREEN}{icon}{RESET}"),
                    _ => skipped = true,
                }
            } else {
                //print!("  ");
                skipped = true;
            }
            print!("{entry}");
            if skipped {
                print!("  ");
            }
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

fn run(
    include_hidden: bool,
    force_col: bool,
    file_icons: HashMap<String, &str>,
    dir: &PathBuf,
) -> Result<(), Box<dyn Error>> {
    if dir.is_dir() {
        let mut files = Vec::new();
        let mut longest_file_name = 0;

        let mut entries = fs::read_dir(dir)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()?;

        entries.sort();

        for entry in entries {
            let mut file_name = entry
                // as by the above iterator file_name will always be valid
                .file_name().unwrap()
                // these lines could be optimized however to_string_lossy replaces invalid characters
                // nicely and i believe this should be faster than handling the error myself
                .to_string_lossy()
                .to_string();
            if entry.metadata()?.is_dir() {
                file_name = file_name + "/";
            }
            if !include_hidden {
                // skip hidden files
                if file_name.chars().nth(0) != Some('.') {
                    longest_file_name =
                        cmp::max(longest_file_name as usize, file_name.chars().count());
                    files.push(file_name);
                }
            } else {
                // include hidden files
                longest_file_name = cmp::max(longest_file_name as usize, file_name.len());
                files.push(file_name);
            }
        }
        output_to_term(files, force_col, longest_file_name, file_icons);
        println!("");
    }
    Ok(())
}

fn run_long(include_hidden: bool, dir: &PathBuf) -> Result<(), Box<dyn Error>> {
    if dir.is_dir() {
        let mut entries = fs::read_dir(dir)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()?;

        entries.sort();

        for entry in entries {
            let entry = entry;
            let file_name = entry
                .file_name().unwrap() // as by above iterator file_name will always be Some
                .to_string_lossy();
            let metadata = entry.metadata()?;
            let size = metadata.len();
            let modified: DateTime<Local> = DateTime::from(metadata.modified()?);
            let mode = metadata.permissions().mode();

            if !include_hidden {
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
