use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Search {
    pub hits: Vec<Hit>,
    pub offset: i64,
    pub limit: i64,
    #[serde(rename = "total_hits")]
    pub total_hits: i64,
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
    pub color: Option<i64>,
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
    pub follows: i64,
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
