use clap::Parser;
use anyhow::{Result};
use serde_json::{Value};
use crate::{file};
use crate::fake::fake_definition::FakeDefinition;

/// `Args` struct is used to parse command line arguments.
/// The `json` field corresponds to the JSON input file.
/// The `count` field specifies how many times the fake data generation should be run.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    json: String,

    #[arg(short, long, default_value_t = 1)]
    count: usize,
}

/// The `start` function is the entry point to the application. It parses the command line arguments,
/// loads the JSON file specified by the arguments, and then calls the `output_json` function to
/// generate and print the fake data.
pub fn start() -> Result<()> {
    let args = Args::parse();
    let fake_definition_json = file::load_json(args.json)?;
    output_json(&fake_definition_json, args.count)
}

/// Takes a JSON value (`fake_definition_json`) and the number of times (`count`) to generate fake data.
/// Depending on the `count`, it decides whether to generate a single set of fake data
/// or multiple sets.
fn output_json(fake_definition_json: &Value, count: usize) -> Result<()> {
    match count {
        1 => output_single_fake_definition(fake_definition_json),
        _ => output_multiply_fake_definition(fake_definition_json, count)
    }
}

/// Generates a single set of fake data from the given JSON value. The resulting fake data is then
/// converted to a pretty-printed JSON string and printed to stdout.
fn output_single_fake_definition(fake_definition_json: &Value) -> Result<()> {
    let fake_definition = FakeDefinition::from_json(fake_definition_json)?;
    let json = serde_json::to_string_pretty(&fake_definition.to_value())?;
    println!("{}", json);
    Ok(())
}

/// The function generates `count` sets of fake data from the given JSON value. Each set of fake data
/// is added to a vector. The vector is then converted to a pretty-printed JSON string and printed to stdout.
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
