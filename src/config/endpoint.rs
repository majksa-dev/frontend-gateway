use serde::Deserialize;

use super::{cache::Cache, method::Method, rewrite::Rewrite};

#[derive(Debug, Clone, Deserialize)]
pub struct Endpoint {
    pub path: String,
    pub id: String,
    pub method: Method,
    pub cache: Option<Cache>,
    pub rewrite: Option<Rewrite>,
}
