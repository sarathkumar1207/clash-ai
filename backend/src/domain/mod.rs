use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerProfile {
    pub tag: String,
    pub name: String,
    pub town_hall_level: u8,
    pub trophies: u32,
    pub clan: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpgradeRecommendation {
    pub target: String,
    pub priority: u8,
    pub reason: String,
    pub estimated_impact: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClanSummary {
    pub tag: String,
    pub name: String,
    pub members: u16,
    pub average_town_hall: f32,
    pub donation_balance: i32,
}
