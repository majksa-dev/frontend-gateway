use anyhow::Result;
use async_trait::async_trait;
use gateway::{Ctx, Next, Request, Response};

pub struct Middleware(super::Context);

impl Middleware {
    pub fn new(context: super::Context) -> Self {
        Self(context)
    }
}

#[async_trait]
impl gateway::Middleware for Middleware {
    async fn run(&self, ctx: &Ctx, mut request: Request, next: Next<'_>) -> Result<Response> {
        let config = match self.0.get(ctx.app_id) {
            Some(config) => config,
            None => {
                return next.run(request).await;
            }
        };
        let endpoint = match config.get(ctx.endpoint_id) {
            Some(config) => config,
            None => {
                return next.run(request).await;
            }
        };
        request.path = endpoint.rewrite.rewrite(request.path);
        next.run(request).await
    }
}
