#[derive(Debug)]
pub enum Rewrite {
    Full(String),
    SearchAndReplace(Vec<Substitution>),
}

#[derive(Debug)]
pub struct Endpoint {
    pub rewrite: Rewrite,
}

impl Endpoint {
    pub fn new(rewrite: Rewrite) -> Self {
        Self { rewrite }
    }
}

#[derive(Debug)]
pub struct Substitution {
    pub from: String,
    pub to: String,
}
