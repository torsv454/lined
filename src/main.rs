extern crate clap;
extern crate lined;

use clap::{App, Arg, ArgGroup};
use lined::{Cfg, ProgramLocation};
use std::error::Error;

fn config() -> Result<Cfg, Box<Error>> {
    let args = App::new("lined")
        .version("0.1")
        .author("Tord Svensson <tord.svensson@gmail.com>")
        .about("A simple non-interactive line editor.")
        .arg(
            Arg::with_name("program")
                .short("p")
                .long("programtext")
                .value_name("text")
                .help("The line editing program to run.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("programfile")
                .value_name("file")
                .help("A file containing the line editing program to run..")
                .takes_value(true),
        )
        .group(
            ArgGroup::with_name("prg")
                .args(&["program", "file"])
                .required(true),
        )
        .get_matches();

    let program = if let Some(text) = args.value_of("program") {
        ProgramLocation::Text(text.to_string())
    } else {
        ProgramLocation::File(args.value_of("file").unwrap().to_string())
    };

    Ok(Cfg { program })
}

fn run() -> Result<(), Box<Error>> {
    let cfg = config()?;
    lined::run(&cfg)?;

    Ok(())
}

fn main() {
    ::std::process::exit(match run() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("Error {}", err.description());
            1
        }
    });
}
