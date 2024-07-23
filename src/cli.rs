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
    count: usize,
}

pub fn start() -> Result<()> {
    let args = Args::parse();
    let fake_definition_json = file::load_json(args.json)?;
    output_json(&fake_definition_json, args.count)
}

fn output_json(fake_definition_json: &Value, count: usize) -> Result<()> {
    match count {
        1 => output_single_fake_definition(fake_definition_json),
        _ => output_multiply_fake_definition(fake_definition_json, count)
    }
}

fn output_single_fake_definition(fake_definition_json: &Value) -> Result<()> {
    let fake_definition = FakeDefinition::from_json(fake_definition_json)?;
    let json = serde_json::to_string_pretty(&fake_definition.to_value())?;
    println!("{}", json);
    Ok(())
}

fn output_multiply_fake_definition(fake_definition_json: &Value, count: usize) -> Result<()> {
    let fake_definitions = (1..=count)
        .map(|_| FakeDefinition::from_json(fake_definition_json))
        .collect::<Result<Vec<FakeDefinition>, _>>()?;

    let values: Vec<_> = fake_definitions
        .iter()
        .map(|fake_definition| { fake_definition.to_value() })
        .collect();

    let json = serde_json::to_string_pretty(&values)?;
    println!("{}", json);

    Ok(())
}
