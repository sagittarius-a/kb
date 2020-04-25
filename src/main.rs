use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;
use std::str;

extern crate clap;
use clap::{App, Arg, ArgMatches};

/// Default activated layouts
const ACTIVATED_LAYOUTS: [&str; 2] = ["us", "fr"];

/// Set the layout to the next layout available
/// It will use a predefined set of layouts if no configuration is found in the
/// LAYOUTS environment variable.
fn next_layout() {
    // Check if user supplied a set of layouts different than the default one
    let key = "LAYOUTS";
    let mut layouts = Vec::new();
    match env::var(key) {
        Ok(val) => {
            for i in val.split(',').collect::<Vec<&str>>() {
                layouts.push(i.to_string());
            }
        }
        Err(_) => {
            for i in &ACTIVATED_LAYOUTS {
                layouts.push((*i).to_string());
            }
        }
    }

    // Find the current layout amongst layouts available if any. Exit otherwise.
    let current = get_layout();
    let index = if let Some(current_layout) = layouts.iter().position(|e| e == &current) {
        current_layout
    } else {
        eprintln!("Current layout not found in available layouts. Exiting");
        return;
    };

    // Apply the new keyboard layout
    match index {
        x if x == layouts.len() - 1 => set_layout(&layouts[0]),
        _ => set_layout(&layouts[index + 1]),
    }
}

/// Write the current layout value to disk. This function does not support the '~'
/// `KEYBOARD_LAYOUT_FILE` environment variable
fn write_layout(layout: &str) {
    // Fetch home directory
    let home_key = "HOME";
    let home;
    match env::var(home_key) {
        Ok(val) => home = val,
        Err(_e) => {
            eprintln!("Could not fetch HOME environment variable");
            return;
        }
    }

    let default_location = Path::new(&home).join(".layout");

    // Check if user supplied a path different than the default one
    let key = "KEYBOARD_LAYOUT_FILE";
    let filepath;
    match env::var(key) {
        Ok(val) => filepath = val,
        Err(_e) => filepath = String::from(default_location.to_str().unwrap()),
    }

    let mut buffer = File::create(Path::new(&filepath)).unwrap();

    buffer.write_all(layout.as_bytes()).unwrap();
}

/// Get the current keyboard layout
fn get_layout() -> String {
    let output = Command::new("setxkbmap")
        .arg("-query")
        .output()
        .expect("failed to execute process");

    let mut current_layout = "";
    for line in str::from_utf8(&output.stdout)
        .unwrap()
        .split('\n')
        .collect::<Vec<&str>>()
    {
        if line.contains("layout") {
            current_layout = line
                .split_whitespace()
                .collect::<Vec<&str>>()
                .last()
                .unwrap();
        }
    }

    String::from(current_layout)
}

/// Set the keyboard layout to a user specified value
fn set_layout(layout: &str) {
    println!("Setting keyboard layout to {}", layout);
    Command::new("setxkbmap")
        .arg(layout)
        .spawn()
        .expect("Failed to execute command to set layout");
    write_layout(layout);
}

/// Manage command line arguments
fn manage_args() -> ArgMatches<'static> {
    App::new("kb")
        .version("1.0,0")
        .author("Sagittarius-a")
        .about("Manage your keyboard layouts easily with Rust & setxkbmap.")
        .arg(
            Arg::with_name("set-layout")
                .short("s")
                .long("set")
                .value_name("LAYOUT")
                .help("Set the keyboard layout to a given value")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("next-layout")
                .short("n")
                .long("next")
                .help(
                    "Set the current keyboard layout to the next layout available. \
                Read the LAYOUTS environment variable. Values must be coma separated, \
                such as 'us,fr'.",
                )
                .takes_value(false),
        )
        .arg(
            Arg::with_name("get-layout")
                .short("g")
                .long("get")
                .help("Get the current keyboard layout")
                .takes_value(false),
        )
        .get_matches()
}

fn main() {
    let matches = manage_args();

    if matches.is_present("get-layout") {
        println!("{}", get_layout());
    } else if matches.value_of("set-layout").is_some() {
        set_layout(matches.value_of("set-layout").unwrap());
    } else if matches.is_present("next-layout") {
        next_layout();
    } else {
        println!("{}", get_layout());
    }
}
