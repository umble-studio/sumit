use serde::{Deserialize, Serialize};

use super::extension::ExtensionHandle;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Manifest {
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

pub trait ExtensionManifest {
    fn manifest(&self) -> Manifest;
}

impl ExtensionManifest for ExtensionHandle<'_> {
    fn manifest(&self) -> Manifest {
        self.manifest.clone()
    }
}