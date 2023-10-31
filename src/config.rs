use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::fs;
use std::io::BufReader;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModConfig {
    pub id: String,
    pub project_name: String,
    pub project_id: String,
    pub name: String,
    pub version_number: String,
    pub channel: String,
    pub mc_version: String,
    pub loader: String,
    pub date_published: String,
    pub filepath: String,
    pub dependents: Option<Vec<String>>,
}

pub fn load_config(directory: &str) -> Result<Vec<ModConfig>, Box<dyn std::error::Error>> {
    let file = fs::File::open(directory.clone())?;
    let reader = BufReader::new(file);
    let config: Vec<ModConfig> = serde_json::from_reader(reader)?;

    Ok(config)
}
