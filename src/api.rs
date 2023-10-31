use reqwest::Response;

use crate::{
    cli_error::CliError, client::get_client, constants::API_URL, project_json::Project,
    version_json::ProjectVersion,
};

pub async fn mod_exists(identifier: &str) -> Result<Response, reqwest::Error> {
    get_client()?
        .get(API_URL.to_string() + "project/" + identifier + "/check")
        .send()
        .await
}

pub async fn find_version(
    identifier: &str,
    mc_loader: &String,
    download_channel: &String,
    mc_version: &String,
) -> Result<Option<ProjectVersion>, Box<dyn std::error::Error>> {
    let res = get_client()?
        .get(API_URL.to_string() + "project/" + identifier)
        .send()
        .await?;
    let project: Project = res.json().await?;

    if project.loaders.contains(mc_loader) && project.game_versions.contains(mc_version) {
        let versions: Vec<ProjectVersion> = get_client()?
            .get(API_URL.to_string() + "project/" + identifier + "/version")
            .send()
            .await?
            .json()
            .await?;

        match &versions
            .iter()
            .find(|version| {
                version.version_type == *download_channel
                    && version.loaders.contains(mc_loader)
                    && version.game_versions.contains(mc_version)
            })
            .cloned()
        {
            Some(version) => Ok(Some(version.clone())),
            None => Err(Box::new(CliError("Version not found".into()))),
        }
    } else {
        Err(Box::new(CliError("Version not found".into())))
    }
}
