use async_trait::async_trait;
use hyper::header::{HeaderName, HeaderValue};
use hyper::StatusCode;
use hyper::{self, Body, HeaderMap};
use hyper::{Client, Request};
#[cfg(feature = "rustls")]
type HttpsConnector = hyper_rustls::HttpsConnector<hyper::client::HttpConnector>;
#[cfg(feature = "rust-native-tls")]
use hyper_tls;
#[cfg(feature = "rust-native-tls")]
type HttpsConnector = hyper_tls::HttpsConnector<hyper::client::HttpConnector>;

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json;

use crate::errors::*;
use crate::issues;
use crate::util::url_join;

use std::cell::RefCell;
use std::sync::Arc;

pub struct YouTrack {
    uri: String,
    token: String,
    client: Arc<Client<HttpsConnector>>,
}

impl Clone for YouTrack {
    fn clone(&self) -> Self {
        Self {
            uri: self.uri.clone(),
            token: self.token.clone(),
            client: Arc::clone(&self.client),
        }
    }
}

new_type!(GetQueryBuilder);
new_type!(PostQueryBuilder);

new_type!(CustomQuery);

exec!(CustomQuery);

#[async_trait]
pub trait Executor {
    async fn execute<T>(self) -> Result<(HeaderMap, StatusCode, Option<T>)>
    where
        T: DeserializeOwned + Send + Sync;
}

impl YouTrack {
    /// Create a new YouTrack client struct. It takes a type that can convert into
    /// an &str (`String` or `Vec<u8>` for example). As long as the function is
    /// given a valid API Token your requests will work.
    pub fn new<T>(uri: T, token: T) -> Result<Self>
    where
        T: ToString,
    {
        let client = Client::builder().build(HttpsConnector::new());
        Ok(Self {
            uri: uri.to_string(),
            token: token.to_string(),
            client: Arc::new(client),
        })
    }

    /// Get the currently set Authorization Token
    pub fn get_token(&self) -> &str {
        &self.token
    }

    /// Change the currently set Authorization Token using a type that can turn
    /// into an &str. Must be a valid API Token for requests to work.
    pub fn set_token<T>(&mut self, token: T)
    where
        T: ToString,
    {
        self.token = token.to_string();
    }

    /// Begin building up a GET request to YouTrack
    pub fn get(&self) -> GetQueryBuilder {
        self.into()
    }

    /// Begin building up a POST request with data to YouTrack

    pub fn post<T>(&self, body: T) -> PostQueryBuilder
    where
        T: Serialize,
    {
        let mut qb: PostQueryBuilder = self.into();

        if let Ok(mut qbr) = qb.request {
            let serialized = serde_json::to_vec(&body);

            match serialized {
                Ok(json) => {
                    *qbr.get_mut().body_mut() = json.into();

                    qb.request = Ok(qbr);
                }

                Err(_) => {
                    qb.request = Err("Unable to serialize data to JSON".into());
                }
            }
        }

        qb
    }
}

impl<'g> GetQueryBuilder<'g> {
    func_client!(custom_endpoint, CustomQuery, endpoint_str);
    func_client!(issues, issues::get::Issues<'g>);
}

impl<'g> PostQueryBuilder<'g> {
    func_client!(custom_endpoint, CustomQuery, endpoint_str);
    func_client!(issues, issues::post::Issues<'g>);
}

// From derivations of Github to the given type using a certain
// request method
from!(
    @GetQueryBuilder
        => "GET"
    @PostQueryBuilder
        => "POST"
);

// Custom Url based from impls
from!(
    @GetQueryBuilder
       => CustomQuery
   @PostQueryBuilder
       => CustomQuery
);

impl<'a> CustomQuery<'a> {
    /// Set custom header for request.
    /// Useful for custom headers (sometimes using in api preview).
    pub fn set_header(
        mut self,
        header_name: impl Into<HeaderName>,
        accept_header: impl Into<HeaderValue>,
    ) -> Self {
        match self.request {
            Ok(mut req) => {
                req.get_mut()
                    .headers_mut()
                    .insert(header_name.into(), accept_header.into());
                self.request = Ok(req);
                self
            }
            Err(_) => self,
        }
    }
}
