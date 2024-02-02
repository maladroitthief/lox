use std::env;
use std::process::exit;

pub mod lox;

fn main() {
    let mut lox = lox::Lox::default();
    let args: Vec<String> = env::args().collect();
    return match args.len() {
        1 => {
            lox.run_prompt();
        }
        2 => {
            lox.run_file(&args[1]);
        }
        _ => {
            println!("Usage: rlox [script]");
            exit(64);
        }
    };
}
