use tabled::Tabled;

use crate::{cli_error::CliError, client::get_client, constants::API_URL, search_json::Search};

#[derive(Tabled)]
pub struct SearchDisplay {
    slug: String,
    client_side: String,
    server_side: String,
    project_type: String,
    downloads: u32,
}

impl SearchDisplay {
    fn new(
        slug: &str,
        client_side: &str,
        server_side: &str,
        project_type: &str,
        downloads: u32,
    ) -> Self {
        Self {
            slug: slug.to_string(),
            client_side: client_side.to_string(),
            server_side: server_side.to_string(),
            project_type: project_type.to_string(),
            downloads,
        }
    }
}

pub async fn search(query: &str) -> Result<Vec<SearchDisplay>, Box<dyn std::error::Error>> {
    let search_result: Option<Search> = match get_client() {
        Ok(client) => Some(
            client
                .get(API_URL.to_owned() + "search?query=" + query)
                .send()
                .await?
                .json()
                .await?,
        ),
        Err(_) => None,
    };

    match search_result {
        Some(search) => Ok(search
            .hits
            .iter()
            .map(|hit| {
                SearchDisplay::new(
                    &hit.slug,
                    &hit.client_side,
                    &hit.server_side,
                    &hit.project_type,
                    hit.downloads,
                )
            })
            .collect()),
        None => Err(Box::new(CliError(
            "API Error could not search for mods".into(),
        ))),
    }
}
