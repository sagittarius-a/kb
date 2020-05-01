![kb](assets/kb.png)

Manage your keyboard layouts easily with Rust & setxkbmap.

## Overview

I wanted to write a bit of Rust, so I ported a Python script I wrote to easily
manage my keyboard layouts.

Since I need to switch keyboard layouts quite often (when I do not have a
custom X11 layout installed), I wrote a wrapper around setxkbmap to switch from
keybaord layout easily.

I designed this utility to be integrated in my xmobar configuration.

I'm sorry for the terrible Rust. I mean it.

## Usage

```sh
kb 1.1.0
Sagittarius-a
Manage your keyboard layouts easily with Rust & setxkbmap.

USAGE:
    kb [FLAGS] [OPTIONS]

FLAGS:
    -g, --get        Get the current keyboard layout
    -h, --help       Prints help information
    -n, --next       Set the current keyboard layout to the next layout available. Read the LAYOUTS environment
                     variable. Values must be coma separated, such as 'us,fr'.
    -V, --version    Prints version information

OPTIONS:
    -s, --set <LAYOUT>    Set the keyboard layout to a given value
```

## Configuration

`kb` is configured thanks to environment variables. Find the list of available
options below:

- `LAYOUTS`: Define a set of layouts available. They must be coma separated, such as 'us,ca' or 'es,pl,us'. One can set it with the command `export LAYOUTS="us,ca,es,pl"`.

- `KEYBOARD_LAYOUT_FILE`: Define the path of the containing the current layout. It does not support the ~ character, so use a full path to avoid any issue.

## Installation

Clone this repository and run `cargo build --release`. You can then add the produced binary to your path.

## License

`kb` is distributed under the terms of the GNU General Public License v3.0.
