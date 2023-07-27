mod lexer;

pub use lexer::Lexer;
use std::io;

fn display_about() {
    println!("TODO: About page")
}

fn display_help() {
    println!("TODO: Help page")
}

fn process_command(cmd: &str) {
    let mut lex = Lexer::new(cmd);

    match lex.tokenize() {
        Ok(tokens) => println!("Execute command {:?}", tokens),
        Err(e) => println!("{}", e),
    }
}

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();

    println!("Terminal calculator");

    loop {
        println!("Enter command:");

        if let Ok(_) = stdin.read_line(&mut line) {
            let trimmed = line.trim();

            match trimmed {
                "quit" | "exit" => break,
                "about" => display_about(),
                "help" => display_help(),
                _ => process_command(trimmed),
            }
        }

        line.clear();
    }
}
