mod tokenizer;
mod pratt;
mod ast;
mod errors;
mod statement;
mod parser;

use std::io::{self, Write};
use parser::build_statement;
use errors::SQLError;

fn main() {
    println!("SQL Parser CLI");
    println!("Type your SQL statement. Type 'exit;' to quit.\n");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("❌ Failed to read input.");
            continue;
        }

        let trimmed = input.trim();
        if trimmed.eq_ignore_ascii_case("exit;") {
            break;
        }

        match build_statement(trimmed) {
            Ok(stmt) => println!("✅ Parsed Statement:\n{:#?}\n", stmt),
            Err(e) => eprintln!("❌ Error: {}\n", e),
        }
    }
}
