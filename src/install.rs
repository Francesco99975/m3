use std::{fs, path::Path};

use async_recursion::async_recursion;
use reqwest::StatusCode;

use crate::{
    api::{find_version, mod_exists},
    client::get_client,
    config::{load_config, ModConfig},
    constants::API_URL,
    models::{ModLoader, VersionChannel},
    project_json::Project,
    version_json::ProjectVersion,
};

pub async fn install(
    directory: &str,
    mod_list: &Vec<String>,
    loader: ModLoader,
    channel: VersionChannel,
    mc_version: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = &(directory.to_owned() + "/.m3.json");
    let mut config = match load_config(config_path.as_str()) {
        Ok(config) => config,
        Err(_) => {
            match fs::write(config_path.as_str(), "") {
                Ok(_) => println!("Created config file"),
                Err(_) => match fs::create_dir(directory) {
                    Ok(_) => match fs::write(config_path.as_str(), "") {
                        Ok(_) => println!("Created mod directory and config file"),
                        Err(_) => eprintln!("Could not create config file"),
                    },
                    Err(_) => eprintln!("Could not created mod directory"),
                },
            };
            Vec::new()
        }
    };

    let mc_loader = match loader {
        ModLoader::Fabric => "fabric".to_string(),
        ModLoader::Forge => "forge".to_string(),
    };

    let download_channel = match channel {
        VersionChannel::Release => String::from("release"),
        VersionChannel::Beta => String::from("beta"),
        VersionChannel::Alpha => String::from("alpha"),
    };
    for _mod in mod_list {
        match mod_exists(_mod).await {
            Ok(res) => {
                if res.status() == StatusCode::OK {
                    match find_version(_mod, &mc_loader, &download_channel, &mc_version).await {
                        Ok(res) => match res {
                            Some(version) => {
                                let file = &version.files[0];
                                let data = reqwest::get(&file.url).await?.bytes().await?;

                                fs::create_dir_all(Path::new(&(directory.to_owned())))?;
                                let filepath = &(directory.to_owned() + "/" + &file.filename);
                                fs::write(Path::new(filepath), data)?;

                                let minecraft_mod = ModConfig {
                                    id: version.id.clone(),
                                    project_name: _mod.to_string(),
                                    project_id: version.project_id.clone(),
                                    version_number: version.version_number.clone(),
                                    name: version.name.clone(),
                                    channel: version.version_type.clone(),
                                    loader: mc_loader.clone(),
                                    mc_version: mc_version.clone(),
                                    date_published: version.date_published.clone(),
                                    filepath: filepath.to_string(),
                                    dependents: None,
                                };

                                config.push(minecraft_mod);

                                match &version.dependencies {
                                    Some(deps) => {
                                        if !deps.is_empty() {
                                            let deps_prj_ids: Vec<&str> = deps
                                                .iter()
                                                .filter(|dep| dep.dependency_type == "required")
                                                .map(|dep| dep.project_id.as_str())
                                                .collect();

                                            let tmp = config.clone();

                                            let installed_prj_ids: Vec<&str> = tmp
                                                .iter()
                                                .map(|mod_config| mod_config.project_id.as_str())
                                                .collect();

                                            let filtered_prj_ids: Vec<&str> = deps_prj_ids
                                                .iter()
                                                .filter(|prj_id| {
                                                    let fltr = installed_prj_ids.contains(prj_id);
                                                    if fltr {
                                                        match config.iter_mut().find(|mod_config| {
                                                            mod_config.project_id == **prj_id
                                                        }) {
                                                            Some(mod_config) => {
                                                                match &mut mod_config.dependents {
                                                                    Some(deps) => {
                                                                        deps.push(_mod.to_string())
                                                                    }
                                                                    None => {
                                                                        mod_config.dependents =
                                                                            Some(vec![
                                                                                _mod.to_string()
                                                                            ])
                                                                    }
                                                                }
                                                            }
                                                            None => eprint!("Mod Config not found"),
                                                        }
                                                    }

                                                    !fltr
                                                })
                                                .copied()
                                                .collect();

                                            if !filtered_prj_ids.is_empty() {
                                                println!(
                                                    "Downloading dependencies for {:?}",
                                                    &_mod
                                                );

                                                download_dependencies(
                                                    &mut config,
                                                    filtered_prj_ids,
                                                    directory,
                                                    &mc_loader,
                                                    &mc_version,
                                                    &_mod.to_string(),
                                                )
                                                .await?;
                                            }
                                        }
                                    }
                                    None => {
                                        println!("No dependencies for {}", &_mod);
                                    }
                                }
                            }
                            None => println!("Version not found"),
                        },
                        Err(_) => eprintln!("Version not found"),
                    }
                }
            }

            Err(_) => eprintln!("Could not find mod: {}", &_mod),
        };
    }

    let json = serde_json::to_string_pretty(&config)?;
    fs::write(config_path, json.as_bytes())?;

    Ok(())
}

#[async_recursion]
async fn download_dependencies(
    config: &mut Vec<ModConfig>,
    deps_prj_ids: Vec<&'async_recursion str>,
    directory: &str,
    mc_loader: &String,
    mc_version: &String,
    parent: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    if deps_prj_ids.is_empty() {
        return Ok(());
    };

    for dep_prj_id in deps_prj_ids {
        let res = get_client()?
            .get(API_URL.to_string() + "project/" + dep_prj_id)
            .send()
            .await?;
        let dependency_project: Project = res.json().await?;

        if dependency_project.loaders.contains(mc_loader)
            && dependency_project.game_versions.contains(mc_version)
        {
            let mut versions: Vec<ProjectVersion> = get_client()?
                .get(API_URL.to_string() + "project/" + dep_prj_id + "/version")
                .send()
                .await?
                .json()
                .await?;

            versions.sort_by(|ver_a, ver_b| {
                let evaluator_a = match ver_a.version_type.as_str() {
                    "release" => 3,
                    "beta" => 2,
                    "alpha" => 1,
                    _ => 0,
                };

                let evaluator_b = match ver_b.version_type.as_str() {
                    "release" => 3,
                    "beta" => 2,
                    "alpha" => 1,
                    _ => 0,
                };

                evaluator_b.cmp(&evaluator_a)
            });

            match &versions
                .iter()
                .find(|version| {
                    version.loaders.contains(mc_loader)
                        && version.game_versions.contains(mc_version)
                })
                .cloned()
            {
                Some(dep_version) => {
                    let dep_file = &dep_version.files[0];
                    let dep_data = reqwest::get(&dep_file.url).await?.bytes().await?;
                    let filepath = &(directory.to_owned() + "/" + &dep_file.filename);
                    fs::write(Path::new(filepath), dep_data)?;

                    let minecraft_mod = ModConfig {
                        id: dep_version.id.clone(),
                        project_name: dependency_project.slug.clone(),
                        project_id: dep_version.project_id.clone(),
                        version_number: dep_version.version_number.clone(),
                        name: dep_version.name.clone(),
                        channel: dep_version.version_type.clone(),
                        loader: mc_loader.clone(),
                        mc_version: mc_version.clone(),
                        date_published: dep_version.date_published.clone(),
                        filepath: filepath.to_string(),
                        dependents: Some(vec![parent.to_string()]),
                    };

                    config.push(minecraft_mod);

                    match &dep_version.dependencies {
                        Some(deps) => {
                            if !deps.is_empty() {
                                download_dependencies(
                                    config,
                                    deps.iter()
                                        .filter(|sub_dep| sub_dep.dependency_type == "required")
                                        .map(|sub_dep| sub_dep.project_id.as_str())
                                        .collect(),
                                    directory,
                                    mc_loader,
                                    mc_version,
                                    &dependency_project.slug.clone(),
                                )
                                .await?;
                            }
                        }
                        None => {
                            eprintln!("No dependencies for {}", &dep_version.name);
                        }
                    }
                }
                None => {
                    eprintln!("No Compatible versions found for dependency");
                }
            }
        }
    }

    Ok(())
}
