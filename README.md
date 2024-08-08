# Rust ls-like tool (rsls)

![image](https://i.imgur.com/PWZcz1q.gif)
rsls is a command line tool for UNIX systems that (almost) mirrors `ls` functionality. I've written it in Rust, so it should be performant enough for daily use.

## Installation

### Dependencies
- You need git to clone this repo. Since you're reading this, I'll assume you already have git installed.

- Rust package manager, [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html). Information about installing/building from source can be found [here](https://doc.rust-lang.org/cargo/).
- [Nerd Font](https://www.nerdfonts.com/font-downloads) icons are used to print file icons to the terminal.
### Cloning the repo
```
git clone https://github.com/joeleehen/rsls
cd rsls
cargo build --release
```
This will build the rsls binary and place it in `/rsls/target/release`. From there, the binary can be copied to `$PATH` or aliased if you want to use it.

The alias could be added to .bashrc, assuming you don't add the rsls binary to $PATH
```
alias rsls=$HOME/rsls/target/release/rsls
```

## Usage
### Note: You must use a Nerd Font in your terminal to properly display the icons!

Using `rsls` will feel familiar to UNIX users.
```
rsls
```
Like `ls`, you can pass flags to change the output or specify a directory to list the contents of.
```
rsls [OPTIONS] [DIR]
```
### Options
Flag | Description
---|---
-h, --help | Display help options
-V, --version | Print version
-l | Include file metadata in listing
-a | Include hidden files
