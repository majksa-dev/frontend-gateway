use gateway::cache;
use serde::Deserialize;
use std::collections::HashMap;

use crate::app::rewrite_static;

use super::{
    app::{AppConfig, AppConfigRaw},
    upstream::Upstream,
};

#[derive(Debug, Clone)]
pub struct Apps {
    pub cdn: Upstream,
    pub apps: HashMap<String, AppConfig>,
}

impl Apps {
    pub fn new(data: String) -> Result<Self, serde_json::Error> {
        Ok(Self::from_raw(AppsRaw::new(data)?))
    }
    pub fn from_raw(data: AppsRaw) -> Self {
        let mut apps = HashMap::new();
        for (name, config) in data.apps {
            apps.insert(name.clone(), AppConfig::from_raw(config, name));
        }
        Apps {
            apps,
            cdn: data.cdn,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppsRaw {
    pub cdn: Upstream,
    pub apps: HashMap<String, AppConfigRaw>,
}

impl AppsRaw {
    pub fn new(data: String) -> Result<Self, serde_json::Error> {
        serde_json::from_str(&data)
    }
}

impl From<&Apps> for cache::Builder {
    fn from(value: &Apps) -> Self {
        value
            .apps
            .iter()
            .map(|(name, app)| {
                (
                    name.clone(),
                    HashMap::<String, cache::config::Endpoint>::from(app),
                )
            })
            .collect()
    }
}

impl From<&Apps> for rewrite_static::Builder {
    fn from(value: &Apps) -> Self {
        value
            .apps
            .iter()
            .map(|(name, app)| {
                (
                    name.clone(),
                    HashMap::<String, rewrite_static::config::Endpoint>::from(app),
                )
            })
            .collect()
    }
}
