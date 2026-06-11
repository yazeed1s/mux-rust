use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaybackRestrictionResponse {
    pub data: PlaybackRestriction,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaybackRestrictionsListResponse {
    pub data: Vec<PlaybackRestriction>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_row_count: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaybackRestriction {
    pub id: String,
    pub created_at: String,
    pub updated_at: String,
    pub referrer: ReferrerRestrictionResponse,
    pub user_agent: UserAgentRestrictionResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReferrerRestrictionResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_no_referrer: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_domains: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserAgentRestrictionResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_high_risk_user_agent: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_no_user_agent: Option<bool>,
}
