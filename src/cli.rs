use clap::Parser;
use anyhow::{Result};
use serde_json::{Value};
use crate::{file};
use crate::fake::fake_definition::FakeDefinition;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    json: String,

    #[arg(short, long, default_value_t = 1)]
    count: u32,
}


pub fn start() -> Result<()> {
    let _args = Args::parse();

    let fake_definition_json = file::load_json(_args.json)?;
    output_json(&fake_definition_json)
}

fn output_json(fake_definition_json: &Value) -> Result<()> {
    let fake_definition = FakeDefinition::from_json(fake_definition_json)?;
    let json = serde_json::to_string_pretty(&fake_definition.to_value())?;
    println!("{}", json);

    Ok(())
}