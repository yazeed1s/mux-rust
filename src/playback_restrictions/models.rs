use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePlaybackRestrictionRequest {
    pub referrer: ReferrerRestriction,
    pub user_agent: UserAgentRestriction,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ListPlaybackRestrictionsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReferrerRestriction {
    pub allowed_domains: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_no_referrer: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserAgentRestriction {
    pub allow_high_risk_user_agent: bool,
    pub allow_no_user_agent: bool,
}
