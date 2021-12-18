use std::fs::read_to_string;

use clap::Parser;
use rustyline::error::ReadlineError;
use rustyline::{Config, Editor, Helper};

use scanner::scan;

use crate::error::LinterError;
use crate::error::{LoxError, LoxResult};
use crate::scanner::tokens::{LoxToken, TokenType};

mod error;
mod linter;
mod scanner;
mod utils;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    file_names: Vec<String>,
}

fn main() {
    let args = Args::parse();

    if args.file_names.is_empty() {
        run_prompt();
    }

    run_files(&args.file_names).unwrap();
}

fn run_prompt() {
    let mut rl = setup_rustyline();
    loop {
        let line = match rl.readline("> ") {
            Ok(l) => l,
            Err(ReadlineError::Eof) => {
                println!("Goodbye!");
                return;
            }
            Err(err) => {
                eprintln!("Error reading line: {:?}", err);
                String::new()
            }
        };
        run(&line, false)
    }
}

fn setup_rustyline() -> Editor<()> {
    let mut rl = Editor::<()>::with_config(Config::builder().auto_add_history(true).build());
    setup_history(&mut rl);
    rl
}

fn setup_history(rl: &mut Editor<impl Helper>) {
    if let Err(e) = rl.load_history(".repl_history") {
        if let Err(e) = std::fs::File::create(".repl_history") {
            eprintln!("Cannot create history file! {}", e)
        }
    }
}

fn run_files(file_names: &[String]) -> anyhow::Result<()> {
    for file_name in file_names {
        let s = read_to_string(file_name)?;
        run(&s, false);
    }
    Ok(())
}

fn run(script: &str, _enable_linting: bool) {
    let tokens = scan(script);

    for token in tokens {
        if let Ok(t) = token {
            println!("{:?}", t);
        } else {
            eprintln!("{:?}", token);
        }
    }
}
