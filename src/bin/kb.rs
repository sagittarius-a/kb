use anyhow::Result;
use clap::{App, Arg, ArgMatches};
use kb::{get_layout, next_layout, set_layout};

/// Manage command line arguments
fn manage_args() -> ArgMatches<'static> {
    App::new("kb")
        .version("1.2.0")
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
            Arg::with_name("quiet")
                .short("q")
                .long("quiet")
                .help("Disable desktop notifications")
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

fn main() -> Result<()> {
    let matches = manage_args();
    let quiet: bool = matches.is_present("quiet");

    if matches.is_present("get-layout") {
        Ok(println!("{}", get_layout()?))
    } else if let Some(layout) = matches.value_of("set-layout") {
        set_layout(layout, quiet)
    } else if matches.is_present("next-layout") {
        next_layout(quiet)
    } else {
        Ok(println!("{}", get_layout()?))
    }
}
