use std::fs::File;
use std::io;
use std::io::Read;

mod cmd;
mod cmds;
mod parser;
mod tokenizer;
use cmd::{Cmd, LineState};
use std::error::Error;
use tokenizer::*;

pub enum ProgramLocation {
    File(String),
    Text(String),
}

pub struct Cfg {
    pub program: ProgramLocation,
}

fn slurp(path: &str) -> Result<String, Box<Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn get_program_text(loc: &ProgramLocation) -> Result<String, Box<Error>> {
    match loc {
        ProgramLocation::Text(text) => Ok(text.to_string()),
        ProgramLocation::File(path) => slurp(path),
    }
}

use std::io::BufRead;

pub fn run(cfg: &Cfg) -> Result<(), Box<Error>> {
    let program_text = get_program_text(&cfg.program)?;
    let program = parser::parse(&mut program_text.chars().tokens()).unwrap();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!("{}", cmd::run(&program, &line.unwrap()));
    }

    Ok(())
}
