use axum::{extract::Path, routing::get, Json, Router};
use crate::{application, domain::PlayerProfile};

pub fn routes() -> Router {
    Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/player/:tag", get(player))
        .route("/coach/:tag", get(coach))
}

async fn player(Path(tag): Path<String>) -> Json<PlayerProfile> {
    Json(PlayerProfile { tag, name: "Demo Chief".into(), town_hall_level: 15, trophies: 5120, clan: Some("ClashAI Labs".into()) })
}

async fn coach(Path(tag): Path<String>) -> Json<Vec<crate::domain::UpgradeRecommendation>> {
    let profile = PlayerProfile { tag, name: "Demo Chief".into(), town_hall_level: 15, trophies: 5120, clan: None };
    Json(application::rank_upgrades(&profile))
}
