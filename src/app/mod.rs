use crate::config::apps::Apps;
use anyhow::{Context, Result};
use bb8_redis::{bb8::Pool, RedisConnectionManager};
use gateway::{self, cache, http::HeaderMapExt, tcp, ParamRouterBuilder, Request, Server};
use http::header;
use std::{net::IpAddr, path::Path};
use tokio::fs;

use crate::env::Env;

async fn create_redis(connection: String) -> Result<Pool<RedisConnectionManager>> {
    let manager = RedisConnectionManager::new(connection)
        .with_context(|| "Failed to create Redis connection manager")?;
    Pool::builder()
        .build(manager)
        .await
        .with_context(|| "Failed to create Redis connection pool")
}

async fn load_config(config_path: impl AsRef<Path>) -> Result<Apps> {
    let config_data = fs::read_to_string(config_path)
        .await
        .with_context(|| "Failed to read config file")?;
    Apps::new(config_data).with_context(|| "Failed to parse config file")
}

fn peer_key_from_host() -> impl Fn(&Request) -> Option<String> + Send + Sync + 'static {
    |req: &Request| {
        req.header(header::HOST)
            .and_then(|host| host.to_str().ok())
            .map(|host| host.to_string())
    }
}

pub async fn build(env: Env) -> Result<Server> {
    let redis_cache = create_redis(env.redis_cache_url).await?;
    let config = load_config(env.config_file).await?;
    let mut builder = gateway::builder(
        tcp::Builder::build(
            config
                .apps
                .iter()
                .map(|(name, app)| {
                    (
                        name.clone(),
                        tcp::config::Connection::new(format!(
                            "{}:{}",
                            app.upstream.host, app.upstream.port
                        )),
                    )
                })
                .collect(),
        ),
        peer_key_from_host(),
    )
    .with_app_port(env.port.unwrap_or(80))
    .with_health_check_port(env.healthcheck_port.unwrap_or(9000))
    .with_host(env.host.unwrap_or(IpAddr::from([127, 0, 0, 1])))
    .register_middleware(
        1,
        cache::Builder::build(
            (&config).into(),
            cache::datastore::RedisDatastore::new(redis_cache),
        ),
    );
    for (peer, config) in config.apps.into_iter() {
        builder = builder.register_peer(
            peer,
            config
                .endpoints
                .into_iter()
                .map(|endpoint| (endpoint.method.into(), endpoint.path, endpoint.id))
                .collect::<ParamRouterBuilder>(),
        );
    }
    builder.build().await
}
