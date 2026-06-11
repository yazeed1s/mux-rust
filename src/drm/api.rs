use crate::{
    common::endpoint,
    drm::{DrmConfigurationResponse, DrmConfigurationsListResponse, ListDrmConfigurationsParams},
    errors::Result,
    http::HttpClient,
};
use std::sync::Arc;

pub struct DrmApi {
    http: Arc<HttpClient>,
}

impl DrmApi {
    pub fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    pub fn list_drm_configurations(
        &self,
        params: Option<ListDrmConfigurationsParams>,
    ) -> Result<DrmConfigurationsListResponse> {
        match params {
            Some(params) => self
                .http
                .get_with_query(endpoint::DRM_CONFIGURATIONS, &params),
            None => self.http.get(endpoint::DRM_CONFIGURATIONS),
        }
    }

    pub fn get_drm_configuration(&self, id: &str) -> Result<DrmConfigurationResponse> {
        self.http.get(&endpoint::drm_configuration(id))
    }
}
