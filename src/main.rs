mod cli;
mod faker_type;
mod error;
mod file;
mod service;

use anyhow::Result;

fn main() -> Result<()> {
    cli::start()
}
