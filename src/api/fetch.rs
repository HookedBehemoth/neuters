use crate::api::{common::ApiResponse, error::ApiError};
use lazy_static::lazy_static;
use rustls::Certificate;
use serde::Deserialize;
use std::sync::Arc;
use ureq::{Agent, AgentBuilder};

lazy_static! {
    static ref AGENT: Agent = {
        let certs = rustls_native_certs::load_native_certs().expect("Could not load certs!");

        let mut root_store = rustls::RootCertStore::empty();
        for cert in certs {
            root_store
                .add(&Certificate(cert.0))
                .expect("Could not add cert!");
        }
        let tls_config = Arc::new(
            rustls::ClientConfig::builder()
                .with_safe_defaults()
                .with_root_certificates(root_store)
                .with_no_client_auth(),
        );

        AgentBuilder::new()
            .tls_config(tls_config)
            .redirects(0)
            .build()
    };
}

pub(crate) fn fetch<T>(url: &str, query: &str) -> Result<T, ApiError>
where
    T: for<'a> Deserialize<'a>,
{
    fn is_success(status: u16) -> bool {
        (200..300).contains(&status)
    }

    let response = match AGENT.get(url).query("query", query).call() {
        Ok(response) => {
            if !is_success(response.status()) {
                return Err(ApiError::External(
                    response.status(),
                    response.into_string().unwrap(),
                ));
            }
            response
        }
        Err(err) => {
            return Err(ApiError::Internal(err.to_string()));
        }
    };

    match response.into_json::<ApiResponse<T>>() {
        Ok(response) => {
            if !is_success(response.status_code) || response.result.is_none() {
                Err(ApiError::External(response.status_code, response.message))
            } else {
                Ok(response.result.unwrap())
            }
        }
        Err(err) => Err(ApiError::Internal(err.to_string())),
    }
}
