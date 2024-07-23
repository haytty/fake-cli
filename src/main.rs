mod cli;
mod file;
mod fake;

use anyhow::Result;

fn main() {
    match cli::start() {
        Ok(_) => (),
        Err(e) => println!("{}", e)
    }
}
