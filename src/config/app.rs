use std::collections::HashMap;

use gateway::cache;
use serde::Deserialize;

use super::{endpoint::Endpoint, upstream::Upstream};

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfigRaw {
    pub upstream: Upstream,
    pub endpoints: Vec<Endpoint>,
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub name: String,
    pub upstream: Upstream,
    pub endpoints: Vec<Endpoint>,
}

impl AppConfig {
    pub fn from_raw(data: AppConfigRaw, name: String) -> Self {
        AppConfig {
            name,
            upstream: data.upstream,
            endpoints: data.endpoints,
        }
    }
}

impl From<&AppConfig> for HashMap<String, cache::config::Endpoint> {
    fn from(value: &AppConfig) -> Self {
        value
            .endpoints
            .iter()
            .filter_map(|endpoint| {
                endpoint
                    .cache
                    .as_ref()
                    .map(|cache| (endpoint.id.clone(), cache::config::Endpoint::from(cache)))
            })
            .collect()
    }
}
