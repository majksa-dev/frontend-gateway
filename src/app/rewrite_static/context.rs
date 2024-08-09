use anyhow::Result;
use async_trait::async_trait;
use essentials::debug;
use futures_util::future::join_all;
use gateway::ConfigToContext;

use super::config;

pub struct Endpoint {
    pub rewrite: Rewrite,
    pub cdn_app: Option<Box<str>>,
}

impl Endpoint {
    pub fn rewrite(&self, path: String) -> String {
        debug!(from = path, "Rewriting path");
        let result = if let Some(app) = self.cdn_app.as_ref() {
            format!("/{}{}", app, self.rewrite.rewrite(path))
        } else {
            self.rewrite.rewrite(path)
        };
        debug!(to = result, "Rewriting path");
        result
    }
}

#[async_trait]
impl ConfigToContext for config::Endpoint {
    type Context = Endpoint;

    async fn into_context(self) -> Result<Self::Context> {
        Ok(Endpoint {
            rewrite: self.rewrite.into_context().await?,
            cdn_app: self.cdn_app.into_context().await?,
        })
    }
}

pub enum Rewrite {
    Full(Box<str>),
    SearchAndReplace(Box<[Substitution]>),
    None,
}

impl Rewrite {
    fn rewrite(&self, path: String) -> String {
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
            Rewrite::None => path,
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
            None => Rewrite::None,
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

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_cdn_rewrite() {
        let endpoint = config::Endpoint {
            rewrite: config::Rewrite::None,
            cdn_app: Some("cdn".into()),
        };
        let endpoint = endpoint.into_context().await.unwrap();
        assert_eq!(endpoint.rewrite("/hello".into()), "/cdn/hello".to_string());
    }

    #[tokio::test]
    async fn test_cdn_rewrite_full() {
        let endpoint = config::Endpoint {
            rewrite: config::Rewrite::Full("endpoint".into()),
            cdn_app: Some("cdn".into()),
        };
        let endpoint = endpoint.into_context().await.unwrap();
        assert_eq!(
            endpoint.rewrite("/hello".into()),
            "/cdn/endpoint".to_string()
        );
    }

    #[tokio::test]
    async fn test_rewrite_full() {
        let rewrite = config::Rewrite::Full("/app".into());
        let rewrite = rewrite.into_context().await.unwrap();
        assert_eq!(rewrite.rewrite("/hello".into()), "/app".to_string());
    }

    #[tokio::test]
    async fn test_rewrite_search_and_replace() {
        let rewrite = config::Rewrite::SearchAndReplace(vec![
            config::Substitution {
                from: "/hello".into(),
                to: "/world".into(),
            },
            config::Substitution {
                from: "/world".into(),
                to: "/hello".into(),
            },
        ]);
        let rewrite = rewrite.into_context().await.unwrap();
        assert_eq!(
            rewrite.rewrite("/hello/world".into()),
            "/hello/hello".to_string()
        );
    }

    #[tokio::test]
    async fn test_rewrite_none() {
        let rewrite = config::Rewrite::None;
        let rewrite = rewrite.into_context().await.unwrap();
        assert_eq!(rewrite.rewrite("/hello".into()), "/hello".to_string());
    }
}
