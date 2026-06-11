use crate::{
    common::endpoint,
    errors::Result,
    http::HttpClient,
    uploads::{
        CreateDirectUploadRequest, DirectUploadResponse, DirectUploadsListResponse,
        ListDirectUploadsParams,
    },
};
use std::sync::Arc;

pub struct UploadsApi {
    http: Arc<HttpClient>,
}

impl UploadsApi {
    pub fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    pub fn create_direct_upload(
        &self,
        body: CreateDirectUploadRequest,
    ) -> Result<DirectUploadResponse> {
        self.http.post(endpoint::UPLOADS, &body)
    }

    pub fn list_direct_uploads(
        &self,
        params: Option<ListDirectUploadsParams>,
    ) -> Result<DirectUploadsListResponse> {
        match params {
            Some(params) => self.http.get_with_query(endpoint::UPLOADS, &params),
            None => self.http.get(endpoint::UPLOADS),
        }
    }

    pub fn get_direct_upload(&self, id: &str) -> Result<DirectUploadResponse> {
        self.http.get(&endpoint::upload(id))
    }

    pub fn cancel_direct_upload(&self, id: &str) -> Result<DirectUploadResponse> {
        self.http.put_empty_response(&endpoint::upload_cancel(id))
    }
}
