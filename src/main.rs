#![allow(dead_code)]

use std::io::{stdin, stdout};

use monkey_interpreter::repl;

fn main() {
    println!("Starting REPL...");

    let stdin = stdin();
    let mut stdout = stdout();

    // Might not be the best to have a lock for so long
    repl::start_repl(&mut stdin.lock(), &mut stdout);
}
