mod error;
mod scanner;

use std::fs::read_to_string;

use anyhow;
use clap::Parser;
use rustyline::{Config, Editor, Helper};

use scanner::scan;

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
        run(&line)
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
    run(&s);
    Ok(())
}

fn run(script: &str) {
    let tokens = scan(script);

    for token in tokens {
        println!("{:?}", token);
    }
}