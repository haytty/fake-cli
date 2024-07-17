use std::collections::BTreeMap;
use std::{fmt, fs};
use clap::Parser;
use fake::{Fake, Faker};
use fake::faker::name::raw::LastName;
use fake::locales::JA_JP;
use anyhow::{Context, Result};
use serde_json::{to_string_pretty, Value};
use crate::faker_type::FakerType;
use crate::{file, service};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    yaml: Option<String>,

    #[arg(short, long)]
    json: String,

    #[arg(short, long, default_value_t = 1)]
    count: u32,
}


pub fn start() -> Result<()> {
    let _args = Args::parse();
   
    let json_data = file::load_json(_args.json)?;
    service::generate_dummy_data::generate_dummy_data(&json_data);

    Ok(())
}