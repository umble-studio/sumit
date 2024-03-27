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

#[derive(Debug, Default)]
pub struct ExtensionRegistry {
    pub manifests: Vec<ExtensionManifest>,
}

impl ExtensionRegistry {
    pub fn register(&mut self, manifest: ExtensionManifest) {
        if let None = Self::is_registered(&self, &manifest.name) {
            self.manifests.push(manifest);
        }
    }

    pub fn unregister(&mut self, name: &str) {
        if let Some(_) = Self::is_registered(&self, name) {
            if let Some(manifest) = self.get_manifest(name) {
                self.manifests.retain(|m| m.name != manifest.name);   
            }
        }
    }

    pub fn get_manifest(&self, name: &str) -> Option<ExtensionManifest> {
        self.manifests.iter().find(|m| m.name == name).cloned()
    }

    fn is_registered(&self, name: &str) -> Option<()> {
        let result = self.manifests.iter().any(|manifest| manifest.name == name);

        if result {
            Some(())
        } else {
            None
        }
    }
}