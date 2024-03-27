#[derive(Debug, Default, Clone)]
pub struct ExtensionManifest {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub license: String,
    pub client: Client,
}

#[derive(Debug, Default, Clone)]
struct Client {
    pub entrypoint: String
}