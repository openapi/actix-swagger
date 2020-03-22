use openapiv3::OpenAPI;
use std::fs;

mod printer;

#[cfg(test)]
pub mod test;

use printer::Printable;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let file_path = "/Users/sergeysova/Projects/authmenow/backend/public-api.openapi.yaml";
    let content = fs::read_to_string(&file_path)?;
    let api: OpenAPI = serde_yaml::from_str(&content)?;

    let mut generated = printer::GeneratedModule::default();

    // println!("{:#?}", api);

    generated.set_name(api.info.title);
    if let Some(description) = api.info.description {
        generated.set_description(description);
    }

    println!("{}", generated.print());

    Ok(())
}
