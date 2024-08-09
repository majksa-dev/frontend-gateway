use http::Method;

use gateway::{Id, Request};

type Route = (Method, String, Id);

#[derive(Debug, Default)]
pub struct Router {
    routes: Vec<Route>,
}

impl FromIterator<Route> for Box<Router> {
    fn from_iter<T: IntoIterator<Item = Route>>(routes: T) -> Self {
        Box::new(Router {
            routes: routes.into_iter().collect(),
        })
    }
}

impl gateway::Router for Router {
    fn matches(&self, request: &Request) -> Option<Id> {
        for (method, path, app_id) in &self.routes {
            if method == request.method
                && request.path.starts_with(path)
                && (request
                    .path
                    .chars()
                    .nth(path.len())
                    .map_or(true, |c| c == '/'))
            {
                return Some(*app_id);
            }
        }
        None
    }
}
