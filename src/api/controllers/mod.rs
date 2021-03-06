use std::fmt::{self, Display};
use std::sync::Arc;

use futures::prelude::*;
use hyper::{header::HeaderValue, header::AUTHORIZATION, Body, HeaderMap, Method, Response, Uri};

use super::error::*;
use models::*;
use services::{KeysService, MetricsService, TransactionsService};

mod fallback;
mod healthcheck;
mod keys;
mod metrics;
mod transactions;

pub use self::fallback::*;
pub use self::healthcheck::*;
pub use self::keys::*;
pub use self::metrics::*;
pub use self::transactions::*;

pub type ControllerFuture = Box<Future<Item = Response<Body>, Error = Error> + Send>;

#[derive(Clone)]
pub struct Context {
    pub body: Vec<u8>,
    pub method: Method,
    pub uri: Uri,
    pub headers: HeaderMap<HeaderValue>,
    pub keys_service: Arc<KeysService>,
    pub transactions_service: Arc<TransactionsService>,
    pub metrics_service: Arc<MetricsService>,
}

impl Context {
    pub fn get_auth_token(&self) -> Option<AuthenticationToken> {
        self.headers
            .get(AUTHORIZATION)
            .and_then(|header| header.to_str().ok())
            .and_then(|header| {
                let len = "Bearer ".len();
                if (header.len() > len) && header.starts_with("Bearer ") {
                    Some(header[len..].to_string())
                } else {
                    None
                }
            })
            .map(|t| AuthenticationToken::new(t))
    }
}

impl Display for Context {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&format!(
            "{} {}, headers: {:#?}, body: {:?}",
            self.method,
            self.uri,
            self.headers,
            String::from_utf8(self.body.clone()).ok()
        ))
    }
}
