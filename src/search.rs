use crate::{client::get_client, constants::API_URL, search_json::Search};

pub async fn search(query: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let search: Search = get_client()
        .expect("Coult not fetch client")
        .get(API_URL.to_owned() + "search?query=" + query)
        .send()
        .await?
        .json()
        .await?;
    let results: Vec<String> = search.hits.iter().map(|hit| hit.slug.to_owned()).collect();

    Ok(results)
}
