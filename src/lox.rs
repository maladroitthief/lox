use std::fs;
use std::io::{self, Write};
use std::process::exit;

pub struct Lox {
    had_error: bool,
}

impl Default for Lox {
    fn default() -> Self {
        Lox { had_error: false }
    }
}

impl Lox {
    pub fn run_file(&self, filename: &String) {
        let source = fs::read_to_string(filename).expect("Unable to read file");
        self.run(source);

        if self.had_error {
            exit(64);
        }
    }

    pub fn run_prompt(&mut self) {
        let stdin = io::stdin();
        let mut stdout = io::stdout();
        let input = &mut String::new();

        loop {
            print!("> ");
            let _ = stdout.flush();
            input.clear();
            match stdin.read_line(input) {
                Ok(_) => {
                    if input.is_empty() {
                        println!("");
                        break;
                    }
                    self.run(input.to_string());
                    self.had_error = false;
                }
                Err(error) => println!("error: {error}"),
            }
        }
    }

    fn run(&self, source: String) {
        println!("{}", source);
    }

    fn error(&self, line: u32, message: &str) {
        self.report(line, "", message);
    }

    fn report(&self, line: u32, err: &str, message: &str) {
        eprintln!("[line {line}] Error{err}: {message}");
    }
}
