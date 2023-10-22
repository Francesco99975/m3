use serde_derive::Deserialize;
use serde_derive::Serialize;

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
    pub downloads: i64,
    #[serde(rename = "icon_url")]
    pub icon_url: Option<String>,
    pub color: Option<i64>,
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
    pub followers: i64,
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
    pub id: String,
    pub name: String,
    pub url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gallery {
    pub url: String,
    pub featured: bool,
    pub title: String,
    pub description: String,
    pub created: String,
    pub ordering: i64,
}
