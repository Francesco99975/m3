use tabled::Tabled;

use crate::config::load_config;

#[derive(Tabled)]
pub struct ListDisplay {
    slug: String,
    version_number: String,
    channel: String,
    loader: String,
    dependency: bool,
}

impl ListDisplay {
    fn new(
        slug: &str,
        version_number: &str,
        channel: &str,
        loader: &str,
        dependency: bool,
    ) -> Self {
        Self {
            slug: slug.to_string(),
            version_number: version_number.to_string(),
            channel: channel.to_string(),
            loader: loader.to_string(),
            dependency,
        }
    }
}

pub fn list(directory: &str) -> Result<Vec<ListDisplay>, Box<dyn std::error::Error>> {
    let config_path = &(directory.to_owned() + "/.m3.json");
    let config = match load_config(config_path.as_str()) {
        Ok(config) => config,
        Err(_) => Vec::new(),
    };

    if config.is_empty() {
        return Ok(vec![]);
    }

    Ok(config
        .iter()
        .map(|cfg| {
            ListDisplay::new(
                &cfg.project_name,
                &cfg.version_number,
                &cfg.channel,
                &cfg.loader,
                cfg.dependents.is_some(),
            )
        })
        .collect())
}
