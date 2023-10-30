use colored::Colorize;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::fs;
use std::io::BufReader;
use std::time::Duration;

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
}

pub fn load_config(directory: &str) -> Result<Vec<ModConfig>, Box<dyn std::error::Error>> {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.blue} {msg}")
            .unwrap()
            .tick_strings(&[
                "▹▹▹▹▹",
                "▸▹▹▹▹",
                "▹▸▹▹▹",
                "▹▹▸▹▹",
                "▹▹▹▸▹",
                "▹▹▹▹▸",
                "▪▪▪▪▪",
            ]),
    );

    pb.set_message(format!("{}", "Checking Confing...".blue().italic()));

    let file = fs::File::open(directory.clone())?;
    let reader = BufReader::new(file);
    let config: Vec<ModConfig> = serde_json::from_reader(reader)?;

    Ok(config)
}
