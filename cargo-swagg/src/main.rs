use openapiv3::OpenAPI;
use std::fs;

mod printer;

#[cfg(test)]
pub mod test;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let file_path = "/Users/sergeysova/Projects/authmenow/backend/public-api.openapi.yaml";
    let content = fs::read_to_string(&file_path)?;
    let api: OpenAPI = serde_yaml::from_str(&content)?;

    // println!("{}", "OAuthAuthorizeRequest".to_snake_case());

    Ok(())
}
