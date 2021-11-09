use clap::{App, AppSettings, ArgSettings, Arg};
use colored::*;
use std::io;
use std::io::Read;
use colored::control::SHOULD_COLORIZE;

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

fn main() -> io::Result<()> {
    let args = App::new("dye")
        .version(VERSION)
        .about("Add color to text.")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(Arg::new("string").setting(ArgSettings::Hidden).min_values(1))

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


    let mut string = args.values_of("string").unwrap().collect::<Vec<&str>>().join(" ");
    if string == "-" {
        string = String::new();
        std::io::stdin().read_to_string(&mut string)?;
    }

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

    SHOULD_COLORIZE.set_override(true);
    println!("{}", colored);
    Ok(())
}