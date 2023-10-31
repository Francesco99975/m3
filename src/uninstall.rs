use std::fs;

use crate::config::load_config;

pub fn uninstall(
    directory: &str,
    mod_list: &[String],
) -> Result<String, Box<dyn std::error::Error>> {
    let config_path = &(directory.to_owned() + "/.m3.json");

    let config = match load_config(config_path.as_str()) {
        Ok(config) => config,
        Err(err) => {
            println!("{:?}", err);
            Vec::new()
        }
    };

    if config.is_empty() {
        return Ok(String::from("Nothing to uninstall..."));
    }
    let mut new_config = config.clone();

    for (index, _mod) in config.iter().enumerate() {
        if mod_list.contains(&_mod.project_name) {
            match &_mod.dependents {
                Some(dependants) => {
                    if mod_list.iter().all(|x| dependants.contains(x)) {
                        match fs::remove_file(&_mod.filepath) {
                            Ok(_) => {
                                println!("{} Mod Uninstalled", _mod.project_name);
                                new_config.remove(index);
                            }
                            Err(_) => eprintln!("Could not uninstall {} Mod", _mod.project_name),
                        }
                    }
                }
                None => match fs::remove_file(&_mod.filepath) {
                    Ok(_) => {
                        println!("{} Mod Uninstalled", _mod.project_name);
                        new_config.remove(index);
                    }
                    Err(_) => eprintln!("Could not uninstall {} Mod", _mod.project_name),
                },
            }
        }
    }

    let json = serde_json::to_string_pretty(&new_config).expect("could not convert to JSON");
    fs::write(config_path, json.as_bytes()).expect("could write JSON");

    Ok(format!(
        "{} Mods Uninstalled",
        config.len() - new_config.len()
    ))
}
