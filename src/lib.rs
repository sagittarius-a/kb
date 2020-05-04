use anyhow::{anyhow, Result};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::str;

extern crate clap;

use notify_rust::{Notification, Timeout};

/// Default activated layouts
const ACTIVATED_LAYOUTS: [&str; 2] = ["us", "fr"];

/// Get the layout provided by the user or gather default configuration
/// Values provided by the user are deduplicated, and the empty value is
/// deleted.
pub fn get_user_layout() -> Vec<String> {
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
    layouts.dedup();
    layouts.retain(|x| x != "");
    layouts
}

/// Set the layout to the next layout available
/// It will use a predefined set of layouts if no configuration is found in the
/// LAYOUTS environment variable.
/// If the LAYOUTS environment variable is set but results in an empty array,
/// an error is raised, since a custom configuration cannot be applied
/// properly
pub fn next_layout(quiet: bool) -> Result<()> {
    let layouts = get_user_layout();
    if layouts.is_empty() {
        Err(anyhow!(
            "Layouts provided in $LAYOUTS results in empty array"
        ))
    } else {
        // Find the current layout amongst layouts available if any. Exit otherwise.
        let current = get_layout()?;
        let index = if let Some(current_layout) = layouts.iter().position(|e| e == &current) {
            current_layout
        } else {
            return Err(anyhow!(
                "Current layout not found in available layouts. Exiting"
            ));
        };

        // Apply the new keyboard layout
        match index {
            x if x == layouts.len() - 1 => set_layout(&layouts[0], quiet)?,
            _ => set_layout(&layouts[index + 1], quiet)?,
        };

        Ok(())
    }
}

/// Write the current layout value to disk. This function does not support the '~'
/// in the `KEYBOARD_LAYOUT_FILE` environment variable
pub fn write_layout(layout: &str) -> Result<()> {
    // Fetch home directory
    let home_key = "HOME";
    let home;
    match env::var(home_key) {
        Ok(val) => home = val,
        Err(_e) => {
            return Err(anyhow!("Could not fetch {} environment variable", home_key));
        }
    }

    let mut default_path = PathBuf::from(&home);
    default_path.push(".layout");
    let default_location = default_path.to_str().unwrap();

    // Check if user supplied a path different than the default one in the
    // dedicated environment variable
    let key = "KEYBOARD_LAYOUT_FILE";
    let filepath: std::string::String;
    match env::var(key) {
        Ok(val) => filepath = val,
        Err(_e) => filepath = default_location.into(),
    }

    // Write the new layout to the appropriate file
    File::create(Path::new(&filepath))?.write_all(layout.as_bytes())?;
    Ok(())
}

/// Get the current keyboard layout
pub fn get_layout() -> Result<String> {
    let command = "setxkbmap";
    let arg = "-query";
    let cmd = match Command::new(command).arg(arg).output() {
        Ok(s) => s,
        Err(_) => {
            return Err(anyhow!(
                "Failed to get current layour with command '{}'",
                format!("{} {}", command, arg)
            ))
        }
    };

    // Make sure the command returned a 0 exit code
    if !cmd.status.success() {
        return Err(anyhow!(
            "Error executing command '{}' with exit code {}. Here is the error message:\n{}",
            format!("{} {}", command, arg),
            cmd.status.code().unwrap(),
            str::from_utf8(&cmd.stderr)?.trim()
        ));
    }

    // Iterate over command line output to find the current layout
    let mut current_layout: Option<&str> = None;
    for line in str::from_utf8(&cmd.stdout)?
        .split('\n')
        .collect::<Vec<&str>>()
    {
        // Extract current layout value
        if line.contains("layout") {
            match line.split_whitespace().collect::<Vec<&str>>().last() {
                Some(l) => current_layout = Some(l),
                None => (),
            }
        }
    }

    match current_layout {
        Some(layout) => Ok(String::from(layout)),
        None => Err(anyhow!("Layout not specified in command output")),
    }
}

/// Set the keyboard layout to a user specified value
pub fn set_layout(layout: &str, quiet: bool) -> Result<()> {
    let command = "setxkbmap";
    let cmd = match Command::new(command).arg(layout).output() {
        Ok(s) => s,
        Err(_) => {
            return Err(anyhow!(
                "Failed to set keyboard layout with command '{}'",
                command
            ))
        }
    };

    // Make sure the command returned a 0 exit code
    if cmd.status.success() {
        write_layout(layout)?;
        if !quiet {
            Notification::new()
                .summary("kb")
                .body(format!("Keyboard layout set to '{}'", layout).as_str())
                .timeout(Timeout::Milliseconds(2000)) //milliseconds
                .show()
                .unwrap();
        }
        Ok(())
    } else {
        Err(anyhow!(
            "Error executing command '{}' with exit code {}. Here is the error message:\n{}",
            command,
            cmd.status.code().unwrap(),
            str::from_utf8(&cmd.stderr)?.trim()
        ))
    }
}
