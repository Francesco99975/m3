use std::fs;

use crate::{config::load_config, progress::CliLoading};

pub fn uninstall(
    directory: &str,
    mod_list: &[String],
) -> Result<String, Box<dyn std::error::Error>> {
    let loading = CliLoading::new();
    loading.set("Loading Config");
    let config_path = &(directory.to_owned() + "/.m3.json");
    let config = match load_config(config_path.as_str()) {
        Ok(config) => config,
        Err(_) => Vec::new(),
    };

    if config.is_empty() {
        loading.end("No Config found...");
        return Ok(String::from("Nothing to uninstall..."));
    }
    let mut new_config = config.clone();
    loading.end("Config loaded!");

    for (index, _mod) in config.iter().enumerate() {
        let uninstall_loading = CliLoading::new();
        uninstall_loading.set(&format!("Uninstalling {}", _mod.project_name));
        if mod_list.contains(&_mod.project_name) {
            match &_mod.dependents {
                Some(dependants) => {
                    if mod_list.iter().all(|x| dependants.contains(x)) {
                        match fs::remove_file(&_mod.filepath) {
                            Ok(_) => {
                                new_config.remove(index);
                                uninstall_loading
                                    .end(&format!("{} Mod Uninstalled", _mod.project_name));
                            }
                            Err(_) => uninstall_loading
                                .end(&format!("Could not uninstall: {}", _mod.project_name)),
                        }
                    }
                }
                None => match fs::remove_file(&_mod.filepath) {
                    Ok(_) => {
                        new_config.remove(index);
                        uninstall_loading.end(&format!("{} Mod Uninstalled", _mod.project_name));
                    }
                    Err(_) => uninstall_loading
                        .end(&format!("Could not uninstall: {}", _mod.project_name)),
                },
            }
        }
    }

    let json = serde_json::to_string_pretty(&new_config)?;
    fs::write(config_path, json.as_bytes())?;

    Ok(format!(
        "{} Mods Uninstalled",
        config.len() - new_config.len()
    ))
}
