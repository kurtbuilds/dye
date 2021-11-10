Links: [Source](https://github.com/kurtbuilds/dye) | [crates.io](https://crates.io/crates/dye-cli)

`dye` is a tool to easily color text in shell.

# Installation

    cargo install dye-cli

# Example Usage

    echo $(dye --red WARN) This tool will knock your socks off.
    
    echo $(dye -r WARN) It takes too many characters to type --red, so every display modifier has a shortcode.
    
    echo $(dye -cl INFO) If your shell supports it, this label will be *blinking*. How cool is that?
    
    echo $(dye --cyan INFO) Lowercase letter options modify the text, uppercase modifies the $(dye -R --black background).
    
    echo $(dye -g SUCCESS) You are a $(dye -u great) engineer for using tools that make your life simpler and easier.

See these commands in action here:

![gif of dye example usage](usage.gif)

Wasn't that fun?

# Documentation

The `--help` option gives you all you need to know. Lowercase short options affect foreground color and style, and 
uppercase short options affect the background.

    Add color to text. Pass text as arguments (like the echo command), or pass no arguments to read
    stdin.

    USAGE:
        dye [OPTIONS] [string]...

    OPTIONS:
        -b, --blue         Set foreground blue
        -B, --bgblue       Set background blue
        -c, --cyan         Set foreground cyan
        -C, --bgcyan       Set background cyan
        -d, --bold         Add bold style
        -g, --green        Set foreground green
        -G, --bggreen      Set background green
        -h, --help         Print help information
        -i, --italic       Add italic style
        -k, --black        Set foreground black
        -K, --bgblack      Set background black
        -l, --blink        Add blink style
        -m, --dimmed       Add dimmed style
        -p, --purple       Set foreground purple
        -P, --bgpurple     Set background purple
        -r, --red          Set foreground red
        -R, --bgred        Set background red
        -u, --underline    Add underline style
        -v, --reversed     Add reversed style
        -V, --version      Print version information
        -w, --white        Set foreground white
        -W, --bgwhite      Set background white
        -y, --yellow       Set foreground yellow
        -Y, --bgyellow     Set background yellow