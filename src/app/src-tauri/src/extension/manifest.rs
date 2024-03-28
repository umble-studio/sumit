use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ExtensionManifest {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub license: String,
    pub client: Client,
    pub server: Option<Server>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Client {
    pub entrypoint: String
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Server {
    pub entrypoint: String
}