use serde::{Deserialize, Serialize};
use std::{fs};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub simulated: bool,
}


impl Config {
    pub fn load() -> Self {
        // Find ~/.config/clust.toml
        let mut path = dirs::config_dir().expect("No config directory found");
        path.push("clust.toml");

        if path.exists() {
            let content = fs::read_to_string(&path)
                .unwrap_or_else(|_| panic!("Could not read {:?}", path));
            toml::from_str(&content)
                .unwrap_or_else(|_| panic!("Invalid TOML in {:?}", path))
        } else {
            // Create default config if not exists
            let default = Config { simulated: false };
            let toml_str = toml::to_string(&default).expect("Could not serialize default config");
            fs::create_dir_all(path.parent().unwrap())
                .expect("Could not create config directory");
            fs::write(&path, toml_str).expect("Could not write default config");
            default

        }
    }
}
