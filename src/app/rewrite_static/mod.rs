use std::collections::HashMap;

use builder::MiddlewareBuilder;
use gateway::{MiddlewareConfig, MiddlewareCtx};

mod builder;
pub mod config;
mod context;
mod middleware;

use middleware::Middleware;

type Config = MiddlewareConfig<(), config::Endpoint>;
type Context = MiddlewareCtx<(), context::Endpoint>;

#[derive(Debug, Default)]
pub struct Builder(HashMap<String, HashMap<String, config::Endpoint>>);

impl Builder {
    pub fn build(self) -> MiddlewareBuilder {
        let config: Config = self
            .0
            .into_iter()
            .map(|(app, config)| (app, ((), config).into()))
            .collect::<HashMap<_, _>>()
            .into();
        MiddlewareBuilder::new(config)
    }
}

impl From<HashMap<String, HashMap<String, config::Endpoint>>> for Builder {
    fn from(auth: HashMap<String, HashMap<String, config::Endpoint>>) -> Self {
        Self(auth)
    }
}

impl<I> FromIterator<(String, I)> for Builder
where
    I: IntoIterator<Item = (String, config::Endpoint)>,
{
    fn from_iter<T: IntoIterator<Item = (String, I)>>(iter: T) -> Self {
        Self(
            iter.into_iter()
                .map(|(key, value)| (key, value.into_iter().collect::<HashMap<_, _>>()))
                .collect(),
        )
    }
}
