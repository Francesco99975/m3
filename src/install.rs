use std::{fs, path::Path};

use reqwest::StatusCode;

use crate::{
    client::get_client, constants::API_URL, project_json::Project, version_json::ProjectVersion,
};

pub async fn install(
    directory: &str,
    mod_list: Vec<&String>,
) -> Result<(), Box<dyn std::error::Error>> {
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

                    if project.loaders.contains(&"fabric".to_string()) {
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
                                version.version_type == "release"
                                    && version.loaders.contains(&"fabric".to_owned())
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
                    }
                }
            }
            Err(_) => eprintln!("Could not find mod: {}", &_mod),
        };
    }

    Ok(())
}
