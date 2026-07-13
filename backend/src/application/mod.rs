use crate::domain::{PlayerProfile, UpgradeRecommendation};

pub fn rank_upgrades(profile: &PlayerProfile) -> Vec<UpgradeRecommendation> {
    let mut items = vec![
        UpgradeRecommendation {
            target: "Heroes".into(),
            priority: 95,
            reason: "Heroes affect every attack plan and war outcome.".into(),
            estimated_impact: "High".into(),
        },
        UpgradeRecommendation {
            target: "Blacksmith equipment".into(),
            priority: 88,
            reason:
                "Equipment upgrades compound hero value without changing war weight dramatically."
                    .into(),
            estimated_impact: "High".into(),
        },
        UpgradeRecommendation {
            target: "Core defenses".into(),
            priority: 76,
            reason: "Protects against common air and hybrid attacks.".into(),
            estimated_impact: "Medium".into(),
        },
    ];
    if profile.town_hall_level < 12 {
        items.retain(|item| item.target != "Blacksmith equipment");
    }
    items
}
