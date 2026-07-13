use crate::domain::{ClanSummary, PlayerProfile};
use async_trait::async_trait;

#[async_trait]
pub trait ClashApi: Send + Sync {
    async fn player(&self, tag: &str) -> anyhow::Result<PlayerProfile>;
    async fn clan(&self, tag: &str) -> anyhow::Result<ClanSummary>;
}

#[derive(Clone)]
pub struct OfficialClashClient {
    base_url: String,
    token: String,
}

impl OfficialClashClient {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            base_url: std::env::var("CLASH_API_BASE_URL")
                .unwrap_or_else(|_| "https://api.clashofclans.com/v1".into()),
            token: std::env::var("CLASH_API_TOKEN")?,
        })
    }
}

#[async_trait]
impl ClashApi for OfficialClashClient {
    async fn player(&self, tag: &str) -> anyhow::Result<PlayerProfile> {
        let encoded = urlencoding::encode(tag);
        let value: serde_json::Value = reqwest::Client::new()
            .get(format!("{}/players/{}", self.base_url, encoded))
            .bearer_auth(&self.token)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(PlayerProfile {
            tag: value["tag"].as_str().unwrap_or(tag).to_string(),
            name: value["name"].as_str().unwrap_or("Unknown").to_string(),
            town_hall_level: value["townHallLevel"].as_u64().unwrap_or_default() as u8,
            trophies: value["trophies"].as_u64().unwrap_or_default() as u32,
            clan: value["clan"]["name"].as_str().map(ToOwned::to_owned),
        })
    }

    async fn clan(&self, tag: &str) -> anyhow::Result<ClanSummary> {
        let encoded = urlencoding::encode(tag);
        let value: serde_json::Value = reqwest::Client::new()
            .get(format!("{}/clans/{}", self.base_url, encoded))
            .bearer_auth(&self.token)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(ClanSummary {
            tag: value["tag"].as_str().unwrap_or(tag).to_string(),
            name: value["name"].as_str().unwrap_or("Unknown clan").to_string(),
            members: value["members"].as_u64().unwrap_or_default() as u16,
            average_town_hall: 0.0,
            donation_balance: 0,
        })
    }
}
