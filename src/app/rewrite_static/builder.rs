use std::collections::HashMap;

use async_trait::async_trait;

use gateway::{Result, Service};

use super::Config;

pub struct MiddlewareBuilder {
    config: Config,
}

impl MiddlewareBuilder {
    pub fn new(config: impl Into<Config>) -> Self {
        Self {
            config: config.into(),
        }
    }
}

#[async_trait]
impl gateway::MiddlewareBuilder for MiddlewareBuilder {
    async fn build(
        self: Box<Self>,
        ids: &[String],
        routers: &HashMap<String, Vec<String>>,
    ) -> Result<Service> {
        Ok(Box::new(super::Middleware::new(
            self.config.into_context(ids, routers).await?,
        )))
    }
}
