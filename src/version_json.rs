use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectVersion {
    pub name: String,
    #[serde(rename = "version_number")]
    pub version_number: String,
    pub changelog: String,
    pub dependencies: Vec<Dependency>,
    #[serde(rename = "game_versions")]
    pub game_versions: Vec<String>,
    #[serde(rename = "version_type")]
    pub version_type: String,
    pub loaders: Vec<String>,
    pub featured: bool,
    pub status: String,
    #[serde(rename = "requested_status")]
    pub requested_status: Option<String>,
    pub id: String,
    #[serde(rename = "project_id")]
    pub project_id: String,
    #[serde(rename = "author_id")]
    pub author_id: String,
    #[serde(rename = "date_published")]
    pub date_published: String,
    pub downloads: i64,
    #[serde(rename = "changelog_url")]
    pub changelog_url: Value,
    pub files: Vec<File>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dependency {
    #[serde(rename = "version_id")]
    pub version_id: String,
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
    pub size: i64,
    #[serde(rename = "file_type")]
    pub file_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hashes {
    pub sha512: String,
    pub sha1: String,
}
