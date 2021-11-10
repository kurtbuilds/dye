extern crate colored;
extern crate clap;

use clap::{App, Arg};
use colored::*;
use std::{io, env};
use std::io::Read;
use colored::control::{SHOULD_COLORIZE};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

/**
Colors:
    black
    red
    green
    yellow
    blue
    magenta (or purple)
    cyan
    white
*/

fn normalize_env(env_res: Result<String, env::VarError>) -> Option<bool> {
    env_res.ok().map(|s| s != "0")
}

fn main() -> io::Result<()> {
    let args = App::new("dye")
        .version(VERSION)
        .about("Add color to text. Pass text as arguments (like the echo command), or pass no arguments to read stdin.")
        .arg(Arg::new("string").min_values(0))

        // foreground
        .arg("-k, --black 'Set foreground black'")
        .arg("-r, --red 'Set foreground red'")
        .arg("-g, --green 'Set foreground green'")
        .arg("-y, --yellow 'Set foreground yellow'")
        .arg("-b, --blue 'Set foreground blue'")
        .arg("-p, --purple 'Set foreground purple'")
        .arg("-c, --cyan 'Set foreground cyan'")
        .arg("-w, --white 'Set foreground white'")

        // style
        .arg("-d, --bold 'Add bold style'")
        .arg("-i, --italic 'Add italic style'")
        .arg("-m, --dimmed 'Add dimmed style'")
        .arg("-l, --blink 'Add blink style'")
        .arg("-u, --underline 'Add underline style'")
        .arg("-v, --reversed 'Add reversed style'")

        // background
        .arg("-K, --bgblack 'Set background black'")
        .arg("-R, --bgred 'Set background red'")
        .arg("-G, --bggreen 'Set background green'")
        .arg("-Y, --bgyellow 'Set background yellow'")
        .arg("-B, --bgblue 'Set background blue'")
        .arg("-P, --bgpurple 'Set background purple'")
        .arg("-C, --bgcyan 'Set background cyan'")
        .arg("-W, --bgwhite 'Set background white'")

        .get_matches();

    let mut print_newline = true;
    let string = match args.values_of("string") {
        Some(x) => x.collect::<Vec<&str>>().join(" "),
        None => {
            print_newline = false;
            let mut buf = String::new();
            std::io::stdin().read_to_string(&mut buf)?;
            buf
        }
    };

    let mut colored = if args.is_present("black") {
        string.black()
    } else if args.is_present("red") {
        string.red()
    } else if args.is_present("green") {
        string.green()
    } else if args.is_present("yellow") {
        string.yellow()
    } else if args.is_present("blue") {
        string.blue()
    } else if args.is_present("purple") {
        string.purple()
    } else if args.is_present("cyan") {
        string.cyan()
    } else if args.is_present("white") {
        string.white()
    } else {
        string.as_str().into()
    };

    if args.is_present("bold") {
        colored = colored.bold();
    }
    if args.is_present("italic") {
        colored = colored.italic();
    }
    if args.is_present("dimmed") {
        colored = colored.dimmed();
    }
    if args.is_present("blink") {
        colored = colored.blink();
    }
    if args.is_present("underline") {
        colored = colored.underline();
    }
    if args.is_present("reversed") {
        colored = colored.reversed();
    }

    colored = if args.is_present("bgblack") {
        colored.on_black()
    } else if args.is_present("bgred") {
        colored.on_red()
    } else if args.is_present("bggreen") {
        colored.on_green()
    } else if args.is_present("bgyellow") {
        colored.on_yellow()
    } else if args.is_present("bgblue") {
        colored.on_blue()
    } else if args.is_present("bgpurple") {
        colored.on_purple()
    } else if args.is_present("bgcyan") {
        colored.on_cyan()
    } else if args.is_present("bgwhite") {
        colored.on_white()
    } else {
        colored
    };

    SHOULD_COLORIZE.set_override(
        if normalize_env(env::var("CLICOLOR_FORCE")) == Some(true) {
            true
        } else if normalize_env(env::var("NO_COLOR")).is_some() {
            false
        } else {
            true
        }
    );

    // newlines are treated differently when inside escape blocks vs outside, so the newline
    // can't be a part of the colored string.
    print!("{}{}", colored, if print_newline { "\n" } else { "" });
    Ok(())
}