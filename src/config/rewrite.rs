use crate::app::rewrite_static;
use serde::Deserialize;

impl From<&Rewrite> for rewrite_static::config::Endpoint {
    fn from(value: &Rewrite) -> Self {
        Self::new(value.into())
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum Rewrite {
    Full(String),
    SearchAndReplace(Vec<Substitution>),
}

impl From<&Rewrite> for rewrite_static::config::Rewrite {
    fn from(value: &Rewrite) -> Self {
        use rewrite_static::config::Rewrite::*;
        match value {
            Rewrite::Full(full) => Full(full.clone()),
            Rewrite::SearchAndReplace(substitutions) => SearchAndReplace(
                substitutions
                    .iter()
                    .map(|substitution| substitution.into())
                    .collect(),
            ),
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
