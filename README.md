# Rust ls-like tool (rsls)

![image](https://i.imgur.com/PWZcz1q.gif)
rsls is a command line tool for UNIX systems that (almost) mirrors `ls` functionality. I've written it in Rust, so it should be performant enough for daily use.

## Installation

### Dependencies

- You need git to clone this repo. Since you're reading this, I'll assume you already have git installed.

- Rust package manager, [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html). Information about installing/building from source can be found [here](https://doc.rust-lang.org/cargo/).
- [Nerd Font](https://www.nerdfonts.com/font-downloads) icons are used to print file icons to the terminal.

### Cloning the repo & installing

```bash
git clone https://github.com/joeleehen/rsls
cd rsls
cargo install
```

This will build the rsls binary and place it in `~/.cargo/bin/`. If you haven't already, cargo will remind you to put that path onto $PATH.

## Usage

### Note: You must use a Nerd Font in your terminal to properly display the icons!

Using `rsls` will feel familiar to UNIX users.

```bash
rsls
```

Like `ls`, you can pass flags to change the output or specify a directory to list the contents of.

```bash
rsls [OPTIONS] [DIR]
```

### Options

Flag | Description
---|---
-h, --help | Display help options
-V, --version | Print version
-l | Include file metadata in listing
-a | Include hidden files
