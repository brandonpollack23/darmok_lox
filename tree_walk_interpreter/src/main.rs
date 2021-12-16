use std::fs::read_to_string;

use anyhow;
use clap::Parser;
use rustyline::{Config, Editor, Helper};

use scanner::scan;

use crate::error::LoxResult;
use crate::scanner::LoxToken;

mod error;
mod scanner;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    script: Option<String>,
}

fn main() {
    let args = Args::parse();

    match args.script {
        Some(file_name) => run_file(&file_name).unwrap(),
        None => run_prompt(),
    }
}

fn run_prompt() {
    let mut rl = setup_rustyline();
    loop {
        let line = match rl.readline("> ") {
            Ok(l) => l,
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
        eprintln!("Error loading repl history: {}", e);
        if let Err(e) = std::fs::File::create(".repl_history") {
            eprintln!("Cannot create history file! {}", e)
        }
    }
}

fn run_file(file_name: &str) -> anyhow::Result<()> {
    let s = read_to_string(file_name)?;
    run(&s, false);
    Ok(())
}

fn run(script: &str, enable_linting: bool) {
    let tokens = scan(script);

    for token in tokens {
        if let Ok(t) = token {
            println!("{:?}", t);
        } else {
            eprintln!("{:?}", token);
        }
    }
}

// TODO linting mode: multiple steps, first just reads token streams for token rules (double space not after newline, use of tabs, etc)
// > handle tab (warning no tabs use spaces)
// > handle non newline whitespace with more than one space (linter warning)
// > warn there should be no \r\n on linux
fn lint(file_name: &str) -> anyhow::Result<()> {
    let script = read_to_string(file_name)?;
    let tokens = scan(&script);
    // let tokenizer_lint_errors = lint(tokens)?;
    // let parser_lint_errors = lint(parse(tokens))?;
    // then print all
    Ok(())
}
