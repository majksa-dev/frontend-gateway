use http::Method;

use gateway::RouterService;

use super::Router;

type Route = (Method, String, String);

#[derive(Debug, Default)]
pub struct RouterBuilder {
    routes: Vec<Route>,
}

impl gateway::RouterBuilder for RouterBuilder {
    fn build(self: Box<Self>) -> (Vec<String>, RouterService) {
        (
            self.routes.iter().map(|(_, _, id)| id.clone()).collect(),
            self.routes
                .into_iter()
                .enumerate()
                .map(|(id, (method, path, _))| (method, path, id))
                .collect::<Box<Router>>(),
        )
    }
}

impl FromIterator<Route> for RouterBuilder {
    fn from_iter<T: IntoIterator<Item = Route>>(iter: T) -> Self {
        Self {
            routes: iter
                .into_iter()
                .map(|mut route| {
                    if route.1.ends_with('/') {
                        route.1.pop();
                    }
                    route
                })
                .collect(),
        }
    }
}

impl From<Vec<Route>> for RouterBuilder {
    fn from(routes: Vec<Route>) -> Self {
        routes.into_iter().collect()
    }
}
