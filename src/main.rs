mod cli;
mod file;
mod fake;

use anyhow::Result;

fn main() -> Result<()> {
    cli::start()
}
