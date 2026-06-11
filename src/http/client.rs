use crate::auth::BasicAuth;
use crate::errors::Result;
use reqwest::blocking::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct HttpClient {
    pub base_url: String,
    client: Client,
    auth: BasicAuth,
}

impl HttpClient {
    pub fn new(base_url: &str, auth: BasicAuth) -> Self {
        Self {
            base_url: base_url.to_string(),
            client: Client::new(),
            auth,
        }
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    pub fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        Ok(self
            .client
            .get(self.url(path))
            .header("Authorization", self.auth.header())
            .send()?
            .error_for_status()?
            .json()?)
    }

    pub fn get_with_query<Q: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        query: &Q,
    ) -> Result<T> {
        Ok(self
            .client
            .get(self.url(path))
            .header("Authorization", self.auth.header())
            .query(query)
            .send()?
            .error_for_status()?
            .json()?)
    }

    pub fn post<B: Serialize, T: DeserializeOwned>(&self, path: &str, body: &B) -> Result<T> {
        Ok(self
            .client
            .post(self.url(path))
            .header("Authorization", self.auth.header())
            .json(body)
            .send()?
            .error_for_status()?
            .json()?)
    }

    pub fn post_empty<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        Ok(self
            .client
            .post(self.url(path))
            .header("Authorization", self.auth.header())
            .send()?
            .error_for_status()?
            .json()?)
    }

    pub fn patch<B: Serialize, T: DeserializeOwned>(&self, path: &str, body: &B) -> Result<T> {
        Ok(self
            .client
            .patch(self.url(path))
            .header("Authorization", self.auth.header())
            .json(body)
            .send()?
            .error_for_status()?
            .json()?)
    }

    pub fn put<B: Serialize, T: DeserializeOwned>(&self, path: &str, body: &B) -> Result<T> {
        Ok(self
            .client
            .put(self.url(path))
            .header("Authorization", self.auth.header())
            .json(body)
            .send()?
            .error_for_status()?
            .json()?)
    }

    pub fn put_empty(&self, path: &str) -> Result<()> {
        self.client
            .put(self.url(path))
            .header("Authorization", self.auth.header())
            .send()?
            .error_for_status()?;
        Ok(())
    }

    pub fn put_empty_response<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        Ok(self
            .client
            .put(self.url(path))
            .header("Authorization", self.auth.header())
            .send()?
            .error_for_status()?
            .json()?)
    }

    pub fn delete(&self, path: &str) -> Result<()> {
        self.client
            .delete(self.url(path))
            .header("Authorization", self.auth.header())
            .send()?
            .error_for_status()?;
        Ok(())
    }
}
