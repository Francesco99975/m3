use std::{fs, path::Path, time::Duration};

use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::StatusCode;

use crate::{
    client::get_client, constants::API_URL, project_json::Project, version_json::ProjectVersion,
};

fn get_mods_to_update(directory: &str) -> Vec<String> {
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

    pb.set_message(format!("{}", "Checking for updates...".blue().italic()));

    let raw_paths = fs::read_dir(directory).expect("Could not open directory");

    let mut mods_paths: Vec<String> = Vec::new();
    for raw_path in raw_paths {
        let path = raw_path.expect("Could not retrieve path").path();

        if path.is_file() && path.ends_with("jar") {
            mods_paths.push(path.to_str().expect("File parsing error").to_string())
        }
    }

    mods_paths
}

pub async fn update_mods(directory: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mods_paths = get_mods_to_update(directory);
    let mods: Vec<String> = mods_paths
        .iter()
        .map(|path| {
            let index = path.find('-').expect("Invalid file");
            path[0..index].to_string()
        })
        .collect();

    for _mod in mods {
        match get_client()
            .expect("Coult not fetch client")
            .get(API_URL.to_string() + "project/" + &_mod + "/check")
            .send()
            .await
        {
            Ok(res) => {
                if res.status() == StatusCode::OK {
                    let res = get_client()
                        .expect("Coult not fetch client")
                        .get(API_URL.to_string() + "project/" + &_mod)
                        .send()
                        .await?;
                    let project: Project = res.json().await?;
                    if project.loaders.contains(&"fabric".to_string()) {
                        let versions: Vec<ProjectVersion> = get_client()
                            .expect("Coult not fetch client")
                            .get(API_URL.to_string() + "project/" + &_mod + "version")
                            .send()
                            .await?
                            .json()
                            .await?;
                        let version = &versions[0].files[0];
                        let data = get_client()
                            .expect("Coult not fetch client")
                            .get(&version.url)
                            .send()
                            .await?
                            .bytes()
                            .await?;

                        match fs::write(
                            Path::new(&(directory.to_owned() + &version.filename)),
                            data,
                        ) {
                            Ok(_) => {
                                for old in &mods_paths {
                                    fs::remove_file(old)?
                                }
                            }
                            Err(_) => eprintln!("Could not updated mods"),
                        }
                    }
                }
            }
            Err(_) => eprintln!("Could not find mod: {}", &_mod),
        };
    }

    Ok(())
}
