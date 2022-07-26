#![allow(dead_code)]

use std::io::{stdin, stdout};

use monkey_interpreter::repl;

fn main() {
    println!("Starting REPL...");

    let stdin = stdin();
    let stdout = stdout();

    repl::start_repl(stdin, stdout);
}
