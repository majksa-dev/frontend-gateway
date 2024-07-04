use serde::Deserialize;

use super::{cache::Cache, method::Method};

#[derive(Debug, Clone, Deserialize)]
pub struct Endpoint {
    pub path: String,
    pub id: String,
    pub cache: Option<Cache>,
    pub method: Method,
}
