use std::{fs, path::Path};

use async_recursion::async_recursion;
use reqwest::StatusCode;

use crate::{
    client::get_client,
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
    let mc_loader = match loader {
        ModLoader::Fabric => "fabric".to_string(),
        ModLoader::Forge => "forge".to_string(),
    };

    let download_channel = match channel {
        VersionChannel::Release => "release",
        VersionChannel::Beta => "beta",
        VersionChannel::Alpha => "alpha",
    };
    for _mod in mod_list {
        match get_client()
            .expect("Coult not fetch client")
            .get(API_URL.to_string() + "project/" + _mod + "/check")
            .send()
            .await
        {
            Ok(res) => {
                if res.status() == StatusCode::OK {
                    let res = get_client()
                        .expect("Coult not fetch client")
                        .get(API_URL.to_string() + "project/" + _mod)
                        .send()
                        .await?;
                    let project: Project = res.json().await?;

                    if project.loaders.contains(&mc_loader)
                        && project.game_versions.contains(&mc_version)
                    {
                        let versions: Vec<ProjectVersion> = get_client()
                            .expect("Coult not fetch client")
                            .get(API_URL.to_string() + "project/" + _mod + "/version")
                            .send()
                            .await?
                            .json()
                            .await?;

                        let version = &versions
                            .iter()
                            .find(|version| {
                                version.version_type == download_channel
                                    && version.loaders.contains(&mc_loader)
                                    && version.game_versions.contains(&mc_version)
                            })
                            .cloned()
                            .expect("Version not found");

                        let file = &version.files[0];
                        let data = reqwest::get(&file.url).await?.bytes().await?;

                        fs::create_dir_all(Path::new(&(directory.to_owned())))
                            .expect("Could not create new mods dir");
                        fs::write(
                            Path::new(&(directory.to_owned() + "/" + &file.filename)),
                            data,
                        )
                        .expect("Could not write file");

                        match &version.dependencies {
                            Some(deps) => {
                                if !deps.is_empty() {
                                    println!("Downloading dependencies for {:?}", &_mod);

                                    let deps_prj_ids = deps
                                        .iter()
                                        .filter(|dep| dep.dependency_type == "required")
                                        .map(|dep| dep.project_id.as_str())
                                        .collect();

                                    download_dependencies(
                                        deps_prj_ids,
                                        directory,
                                        &mc_loader,
                                        &mc_version,
                                    )
                                    .await?
                                }
                            }
                            None => println!("No dependencies for {}", &_mod),
                        }
                    } else {
                        println!("This {} did not match the reqested parameters", &_mod)
                    }
                }
            }
            Err(_) => eprintln!("Could not find mod: {}", &_mod),
        };
    }

    Ok(())
}

#[async_recursion]
async fn download_dependencies(
    deps_prj_ids: Vec<&'async_recursion str>,
    directory: &str,
    mc_loader: &String,
    mc_version: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    if deps_prj_ids.is_empty() {
        return Ok(());
    };

    for dep_prj_id in deps_prj_ids {
        let res = get_client()
            .expect("Coult not fetch client")
            .get(API_URL.to_string() + "project/" + dep_prj_id)
            .send()
            .await?;
        let dependency_project: Project = res.json().await?;

        if dependency_project.loaders.contains(mc_loader)
            && dependency_project.game_versions.contains(mc_version)
        {
            let mut versions: Vec<ProjectVersion> = get_client()
                .expect("Coult not fetch client")
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

            let dep_version = &versions
                .iter()
                .find(|version| {
                    version.loaders.contains(mc_loader)
                        && version.game_versions.contains(mc_version)
                })
                .cloned()
                .expect("Version not found");

            let dep_file = &dep_version.files[0];
            let dep_data = reqwest::get(&dep_file.url).await?.bytes().await?;

            fs::write(
                Path::new(&(directory.to_owned() + "/" + &dep_file.filename)),
                dep_data,
            )
            .expect("Could not write dependency file");

            match &dep_version.dependencies {
                Some(deps) => {
                    if !deps.is_empty() {
                        download_dependencies(
                            deps.iter()
                                .filter(|sub_dep| sub_dep.dependency_type == "required")
                                .map(|sub_dep| sub_dep.project_id.as_str())
                                .collect(),
                            directory,
                            mc_loader,
                            mc_version,
                        )
                        .await?
                    }
                }
                None => todo!(),
            }
        }
    }

    Ok(())
}
