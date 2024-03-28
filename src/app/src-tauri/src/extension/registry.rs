use dirs::document_dir;
use glob::glob;
use std::path::Path;

use super::{constant::EXTENSION_DIRECTORY, manifest::Manifest};

#[derive(Debug, Default)]
pub struct ExtensionRegistry {
    pub manifests: Vec<(String, Manifest)>,
}

impl ExtensionRegistry {
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
                    // println!("Found: {}", path.display());

                    if let Some(prefix) = path.file_stem() {
                        if prefix == "manifest" {
                            println!("Loading manifest: {}", path.display().to_string());

                            let path = path.to_str().unwrap().to_string();
                            paths.push(path.clone());

                            if let Some(manifest) = self.get_manifest(path.as_str()) {
                                let manifest_clone = manifest.clone();

                                if !self.is_registered(path.as_str()) {
                                    self.manifests.push((path.to_string(), manifest));
                                    println!("Register manifest: {}", manifest_clone.name);
                                } else {
                                    println!(
                                        "Manifest already registered: {}",
                                        manifest_clone.name
                                    );
                                }
                            } else {
                                println!("Failed to get manifest: {}", path.as_str());
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

    pub fn get_manifest(&self, manifest_path: &str) -> Option<Manifest> {
        let manifest_path = Path::new(manifest_path);

        if manifest_path.is_file() {
            if let Ok(content) = std::fs::read_to_string(manifest_path) {
                if let Ok(manifest) = serde_json::from_str::<Manifest>(content.as_str()) {
                    Some(manifest)
                } else {
                    println!(
                        "Failed to parse manifest: {}",
                        manifest_path.to_str().unwrap()
                    );
                    None
                }
            } else {
                println!(
                    "Failed to read manifest file: {}",
                    manifest_path.to_str().unwrap()
                );
                None
            }
        } else {
            None
        }
    }

    pub fn get_manifests(&self) -> Vec<(String, Manifest)> {
        self.manifests.clone()
    }

    fn is_registered(&self, manifest_path: &str) -> bool {
        self.manifests
            .iter()
            .any(|manifest| manifest.0 == manifest_path)
    }
}
