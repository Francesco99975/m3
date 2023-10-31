use std::fs;

use crate::{
    config::{load_config, ModConfig},
    progress::CliLoading,
};

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

    let dependencies: Vec<&ModConfig> = config.iter().filter(|x| x.dependents.is_some()).collect();

    for (index, _mod) in config.iter().enumerate() {
        let uninstall_loading = CliLoading::new();
        uninstall_loading.set(&format!("Uninstalling {}", _mod.project_name));
        if mod_list.contains(&_mod.project_name) {
            match fs::remove_file(&_mod.filepath) {
                Ok(_) => {
                    new_config.remove(index);
                    uninstall_loading.end(&format!("{} Mod Uninstalled", _mod.project_name));
                }
                Err(_) => {
                    uninstall_loading.end(&format!("Could not uninstall: {}", _mod.project_name))
                }
            };
        }
    }

    for dep in dependencies.iter() {
        let uninstall_loading = CliLoading::new();
        uninstall_loading.set(&format!(
            "Uninstalling unused dependedncies {}",
            dep.project_name
        ));
        match &dep.dependents {
            Some(dpts) => {
                if !new_config
                    .iter()
                    .any(|cfg| dpts.contains(&cfg.project_name))
                {
                    match fs::remove_file(&dep.filepath) {
                        Ok(_) => {
                            new_config.retain(|cfg| cfg.id != dep.id);
                            uninstall_loading.end(&format!("{} Mod Uninstalled", dep.project_name))
                        }
                        Err(_) => uninstall_loading
                            .end(&format!("Could not uninstall: {}", dep.project_name)),
                    };
                }
            }
            None => uninstall_loading.end(&format!("{} Mod Uninstalled", dep.project_name)),
        }
    }

    let json = serde_json::to_string_pretty(&new_config)?;
    fs::write(config_path, json.as_bytes())?;

    Ok(format!(
        "{} Mods Uninstalled",
        config.len() - new_config.len()
    ))
}
