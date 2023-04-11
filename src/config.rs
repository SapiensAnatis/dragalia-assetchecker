use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub assetpaths: Vec<String>,
}
