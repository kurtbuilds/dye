`dye` is a tool to easily colorize text in shell.

# Installation

    cargo install dye-cli

# Usage

    echo $(dye --red WARN) This tool will knock your socks off.
    echo $(dye -r WARN) It takes too many characters to type --red, so every display modifier has a shortcode.
    echo $(dye -cl INFO) If your shell supports it, this label will be *blinking*. How cool is that?
    echo $(dye --cyan INFO) Lowercase letter options modify the text, uppercase modifies the $(dye -R --black background).
    echo $(dye -g SUCCESS) You are a $(dye -u great) engineer for using tools that make your life simpler and easier.
    
Wasn't that fun?