use std::error::Error;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Label {
    name: String,
    color: String,
    description: String,
}

///
/// Extract labels from a JSON file in the data directory.
///
pub fn extract_default() -> Result<Vec<Label>, Box<dyn Error>> {
    let file = std::fs::read_to_string("data/labels.json")?;
    let labels: Vec<Label> = serde_json::from_str(&file)?;

    Ok(labels)
}

///
/// Extract labels from a JSON file in a specified path.
///
pub fn extract_from_path(path: &str) -> Result<Vec<Label>, Box<dyn Error>> {
    let file = std::fs::read_to_string(path)?;
    let labels: Vec<Label> = serde_json::from_str(&file)?;

    Ok(labels)
}
