use anyhow::Result;
use async_trait::async_trait;
use futures_util::future::join_all;
use gateway::ConfigToContext;

use super::config;

pub struct Endpoint {
    pub rewrite: Rewrite,
}

#[async_trait]
impl ConfigToContext for config::Endpoint {
    type Context = Endpoint;

    async fn into_context(self) -> Result<Self::Context> {
        Ok(Endpoint {
            rewrite: self.rewrite.into_context().await?,
        })
    }
}

pub enum Rewrite {
    Full(Box<str>),
    SearchAndReplace(Box<[Substitution]>),
}

impl Rewrite {
    pub fn rewrite(&self, path: String) -> String {
        match self {
            Rewrite::Full(replacement) => {
                if replacement.starts_with('/') {
                    replacement.to_string()
                } else {
                    format!("/{}", replacement)
                }
            }
            Rewrite::SearchAndReplace(substitutions) => {
                substitutions.iter().fold(path, |path, substitution| {
                    path.replace(substitution.get_from(), substitution.get_to())
                })
            }
        }
    }
}

#[async_trait]
impl ConfigToContext for config::Rewrite {
    type Context = Rewrite;

    async fn into_context(self) -> Result<Self::Context> {
        use config::Rewrite::*;
        Ok(match self {
            Full(replacement) => Rewrite::Full(replacement.into_context().await?),
            SearchAndReplace(substitutions) => Rewrite::SearchAndReplace(
                join_all(
                    substitutions
                        .into_iter()
                        .map(|substitution| substitution.into_context()),
                )
                .await
                .into_iter()
                .collect::<Result<Box<[_]>>>()?,
            ),
        })
    }
}

pub struct Substitution(Box<str>, usize);

impl Substitution {
    pub fn new(mut from: String, to: String) -> Self {
        let len = from.len();
        from.push_str(&to);
        Substitution(from.into_boxed_str(), len)
    }

    pub fn get_from(&self) -> &str {
        &self.0[..self.1]
    }

    pub fn get_to(&self) -> &str {
        &self.0[self.1..]
    }
}

#[async_trait]
impl ConfigToContext for config::Substitution {
    type Context = Substitution;

    async fn into_context(self) -> Result<Self::Context> {
        Ok(Substitution::new(self.from, self.to))
    }
}
