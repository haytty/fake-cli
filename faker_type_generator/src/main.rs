use std::collections::HashMap;
use std::env;
use tera::{Context, Tera};
use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let current_dir = env::current_dir()?;
    let template_dir = current_dir.join("templates/*.tmpl");
    println!("{}", template_dir.to_str().ok_or(anyhow!("hoge"))?);

    let tera = Tera::new(&template_dir.to_string_lossy())?;

    println!("{:?}", tera);

    let mut context = Context::new();

    let mut string_faker_types = HashMap::new();
    string_faker_types.insert("word".to_string(), "Word".to_string());

    let mut array_faker_types = HashMap::new();
    array_faker_types.insert("words".to_string(), "Words".to_string());

    let mut number_faker_types = HashMap::new();
    number_faker_types.insert("digit".to_string(), "Digit".to_string());

    let mut faker_types = string_faker_types.clone();
    faker_types.extend(array_faker_types.clone());
    faker_types.extend(number_faker_types.clone());

    context.insert("faker_types", &faker_types);
    context.insert("string_faker_types", &string_faker_types);
    context.insert("array_faker_types", &array_faker_types);
    context.insert("number_faker_types", &number_faker_types);

    let a = tera.render("faker_type.tmpl", &context)?;

    println!("{}", a);

    Ok(())
}
