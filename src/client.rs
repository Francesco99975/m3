static APP_USER_AGENT: &str = "https://github.com/Francesco99975/m3";

pub fn get_client() -> Result<reqwest::Client, reqwest::Error> {
    reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
}
