use crate::{
    common::endpoint,
    delivery_usage::{DeliveryUsageParams, DeliveryUsageResponse},
    errors::Result,
    http::HttpClient,
};
use std::sync::Arc;

pub struct DeliveryUsageApi {
    http: Arc<HttpClient>,
}

impl DeliveryUsageApi {
    pub fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    pub fn list_delivery_usage(
        &self,
        params: Option<DeliveryUsageParams>,
    ) -> Result<DeliveryUsageResponse> {
        match params {
            Some(params) => self.http.get_with_query(endpoint::DELIVERY_USAGE, &params),
            None => self.http.get(endpoint::DELIVERY_USAGE),
        }
    }
}
