use crate::app::rewrite_static;
use serde::Deserialize;

use super::{app::AppConfig, endpoint::Endpoint};

impl From<(&AppConfig, &Endpoint)> for rewrite_static::config::Endpoint {
    fn from((app, endpoint): (&AppConfig, &Endpoint)) -> Self {
        Self::new(
            endpoint.rewrite.as_ref().into(),
            if app.upstream.is_none() {
                Some(app.name.clone())
            } else {
                None
            },
        )
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum Rewrite {
    Full(String),
    SearchAndReplace(Vec<Substitution>),
}

impl From<Option<&Rewrite>> for rewrite_static::config::Rewrite {
    fn from(value: Option<&Rewrite>) -> Self {
        use rewrite_static::config::Rewrite::*;
        match value {
            Some(Rewrite::Full(full)) => Full(full.clone()),
            Some(Rewrite::SearchAndReplace(substitutions)) => SearchAndReplace(
                substitutions
                    .iter()
                    .map(|substitution| substitution.into())
                    .collect(),
            ),
            Option::None => None,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Substitution {
    pub from: String,
    pub to: String,
}

impl From<&Substitution> for rewrite_static::config::Substitution {
    fn from(value: &Substitution) -> Self {
        Self {
            from: value.from.clone(),
            to: value.to.clone(),
        }
    }
}
