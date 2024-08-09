#[derive(Debug)]
pub enum Rewrite {
    Full(String),
    SearchAndReplace(Vec<Substitution>),
    None,
}

#[derive(Debug)]
pub struct Endpoint {
    pub rewrite: Rewrite,
    pub cdn_app: Option<String>,
}

impl Endpoint {
    pub fn new(rewrite: Rewrite, cdn_app: Option<String>) -> Self {
        Self { rewrite, cdn_app }
    }
}

#[derive(Debug)]
pub struct Substitution {
    pub from: String,
    pub to: String,
}
