use std::{fs, path::Path};

use reqwest::StatusCode;

use crate::{
    api::{find_version, mod_exists},
    config::{load_config, ModConfig},
    progress::CliLoading,
};

pub async fn update_mods(directory: &str) -> Result<String, Box<dyn std::error::Error>> {
    let loading = CliLoading::new();
    loading.set("Loading Config");
    let config_path = &(directory.to_owned() + "/.m3.json");
    let mut config = match load_config(config_path.as_str()) {
        Ok(config) => config,
        Err(_) => Vec::new(),
    };

    if config.is_empty() {
        loading.end("No Config found...");
        return Ok(String::from("Nothing to update..."));
    }

    loading.end("Config loaded!");

    for mut _mod in config.iter_mut() {
        let update_loading = CliLoading::new();
        update_loading.set(&format!("Updating {}", _mod.project_name));
        match mod_exists(_mod.id.as_str()).await {
            Ok(res) => {
                if res.status() == StatusCode::OK {
                    match find_version(
                        _mod.id.as_str(),
                        &_mod.loader,
                        &_mod.channel,
                        &_mod.mc_version,
                    )
                    .await
                    {
                        Ok(version) => match version {
                            Some(version) => {
                                if _mod.version_number != version.version_number {
                                    let file = &version.files[0];
                                    let data = reqwest::get(&file.url).await?.bytes().await?;
                                    let filepath = &(directory.to_owned() + "/" + &file.filename);
                                    fs::write(Path::new(filepath), data)?;

                                    let mut minecraft_mod = ModConfig {
                                        id: version.id.clone(),
                                        project_name: _mod.project_name.clone(),
                                        project_id: version.project_id.clone(),
                                        version_number: version.version_number.clone(),
                                        name: version.name.clone(),
                                        channel: version.version_type.clone(),
                                        loader: _mod.loader.clone(),
                                        mc_version: _mod.mc_version.clone(),
                                        date_published: version.date_published.clone(),
                                        filepath: filepath.to_string(),
                                        dependents: _mod.dependents.clone(),
                                    };
                                    _mod = &mut minecraft_mod;

                                    update_loading.end(&format!("{} Updated!", _mod.project_name));
                                }
                            }
                            None => {
                                update_loading.end(&format!(
                                    "Compatible Version not found for {}",
                                    _mod.project_name
                                ));
                            }
                        },
                        Err(_) => update_loading
                            .end(&format!("Could not find this Mod: {}", _mod.project_name)),
                    }
                }
            }
            Err(_) => {
                update_loading.end(&format!("Could not find this Mod: {}", _mod.project_name))
            }
        };
    }

    let json = serde_json::to_string_pretty(&config)?;
    fs::write(config_path, json.as_bytes())?;

    Ok(String::from("Mods Successfully Updated!"))
}
