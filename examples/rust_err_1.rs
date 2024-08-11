// FROM HERE
// https://stevedonovan.github.io/rust-gentle-intro/6-error-handling.html

// box-error.rs
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

fn run(file: &str) -> Result<i32, Box<dyn Error>> {
    let mut file = File::open(file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.trim().parse()?)
}

fn main() {
    let result_run = run("not_availble.txt");
    let file_name = match result_run {
        Ok(file_name) => file_name,
        Err(e) => Err(e),
    };
}

// cargo run --example rust_err_1
