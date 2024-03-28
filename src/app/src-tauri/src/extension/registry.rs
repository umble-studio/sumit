use std::path::Path;
use dirs::document_dir;
use glob::glob;

use super::{constant::EXTENSION_DIRECTORY, manifest::ExtensionManifest};

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

    pub fn load_manifests(&mut self) -> Vec<String> {
        self.manifests.clear();

        let document_dir = document_dir().unwrap();
        let extension_dir = Path::join(&document_dir, EXTENSION_DIRECTORY);
        let pattern = &format!("{}/**/*.json", extension_dir.to_str().unwrap().to_string());

        println!("Looking for extension manifests in: {}", pattern);

        let mut paths: Vec<String> = Vec::new();

        for entry in glob(pattern).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    println!("Found: {}", path.display());

                    if let Some(prefix) = path.file_stem() {
                        if prefix == "manifest" {
                            println!("Loading manifest: {}", prefix.to_str().unwrap().to_string());

                            let path = path.to_str().unwrap().to_string();
                            paths.push(path.clone());

                            if let Some(manifest) = self.get_manifest(path.as_str()) {
                                self.manifests.push(manifest);
                            }
                        }   
                    }
                }
                Err(err) => {
                    println!("Error: {}", err);
                }
            }
        }

        paths
    }

    pub fn get_manifests(&self) -> Vec<ExtensionManifest> {
        self.manifests.clone()
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