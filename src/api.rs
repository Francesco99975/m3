use reqwest::Response;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

use crate::{cli_error::CliError, client::get_client};

pub const API_URL: &str = "https://api.modrinth.com/v2/";

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub categories: Vec<String>,
    #[serde(rename = "client_side")]
    pub client_side: String,
    #[serde(rename = "server_side")]
    pub server_side: String,
    pub body: String,
    pub status: String,
    #[serde(rename = "requested_status")]
    pub requested_status: Option<String>,
    #[serde(rename = "additional_categories")]
    pub additional_categories: Option<Vec<String>>,
    #[serde(rename = "issues_url")]
    pub issues_url: Option<String>,
    #[serde(rename = "source_url")]
    pub source_url: Option<String>,
    #[serde(rename = "wiki_url")]
    pub wiki_url: Option<String>,
    #[serde(rename = "discord_url")]
    pub discord_url: Option<String>,
    #[serde(rename = "donation_urls")]
    pub donation_urls: Option<Vec<DonationUrl>>,
    #[serde(rename = "project_type")]
    pub project_type: String,
    pub downloads: u32,
    #[serde(rename = "icon_url")]
    pub icon_url: Option<String>,
    pub color: Option<u32>,
    #[serde(rename = "thread_id")]
    pub thread_id: Option<String>,
    #[serde(rename = "monetization_status")]
    pub monetization_status: Option<String>,
    pub id: String,
    pub team: String,
    #[serde(rename = "body_url")]
    pub body_url: Option<String>,
    #[serde(rename = "moderator_message")]
    pub moderator_message: Option<String>,
    pub published: String,
    pub updated: String,
    pub approved: Option<String>,
    pub queued: Option<String>,
    pub followers: u32,
    pub license: Option<License>,
    pub versions: Vec<String>,
    #[serde(rename = "game_versions")]
    pub game_versions: Vec<String>,
    pub loaders: Vec<String>,
    pub gallery: Option<Vec<Gallery>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DonationUrl {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct License {
    pub id: Option<String>,
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gallery {
    pub url: String,
    pub featured: bool,
    pub title: Option<String>,
    pub description: Option<String>,
    pub created: String,
    pub ordering: Option<i32>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Search {
    pub hits: Vec<Hit>,
    pub offset: i32,
    pub limit: i32,
    #[serde(rename = "total_hits")]
    pub total_hits: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hit {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub categories: Option<Vec<String>>,
    #[serde(rename = "client_side")]
    pub client_side: String,
    #[serde(rename = "server_side")]
    pub server_side: String,
    #[serde(rename = "project_type")]
    pub project_type: String,
    pub downloads: u32,
    #[serde(rename = "icon_url")]
    pub icon_url: Option<String>,
    pub color: Option<u32>,
    #[serde(rename = "thread_id")]
    pub thread_id: Option<String>,
    #[serde(rename = "monetization_status")]
    pub monetization_status: Option<String>,
    #[serde(rename = "project_id")]
    pub project_id: String,
    pub author: String,
    #[serde(rename = "display_categories")]
    pub display_categories: Option<Vec<String>>,
    pub versions: Vec<String>,
    pub follows: u32,
    #[serde(rename = "date_created")]
    pub date_created: String,
    #[serde(rename = "date_modified")]
    pub date_modified: String,
    #[serde(rename = "latest_version")]
    pub latest_version: Option<String>,
    pub license: String,
    pub gallery: Vec<String>,
    #[serde(rename = "featured_gallery")]
    pub featured_gallery: Option<String>,
    pub dependencies: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectVersion {
    pub name: String,
    #[serde(rename = "version_number")]
    pub version_number: String,
    pub changelog: Option<String>,
    pub dependencies: Option<Vec<Dependency>>,
    #[serde(rename = "game_versions")]
    pub game_versions: Vec<String>,
    #[serde(rename = "version_type")]
    pub version_type: String,
    pub loaders: Vec<String>,
    pub featured: bool,
    pub status: Option<String>,
    #[serde(rename = "requested_status")]
    pub requested_status: Option<String>,
    pub id: String,
    #[serde(rename = "project_id")]
    pub project_id: String,
    #[serde(rename = "author_id")]
    pub author_id: String,
    #[serde(rename = "date_published")]
    pub date_published: String,
    pub downloads: u32,
    #[serde(rename = "changelog_url")]
    pub changelog_url: Value,
    pub files: Vec<File>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dependency {
    #[serde(rename = "version_id")]
    pub version_id: Option<String>,
    #[serde(rename = "project_id")]
    pub project_id: String,
    #[serde(rename = "file_name")]
    pub file_name: Option<String>,
    #[serde(rename = "dependency_type")]
    pub dependency_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub hashes: Hashes,
    pub url: String,
    pub filename: String,
    pub primary: bool,
    pub size: u32,
    #[serde(rename = "file_type")]
    pub file_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hashes {
    pub sha512: Option<String>,
    pub sha1: Option<String>,
}

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
