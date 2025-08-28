use serde::{Deserialize, Serialize};
use std::{fs};
use whoami;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub simulated: bool,
    pub kube: KubeConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KubeConfig {
    pub default_ctx: String,
    pub clusters: Vec<String>,
    pub users: Vec<String>,
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
                .unwrap_or_else(|e| panic!("Invalid TOML in {:?}: {}", path, e))
        } else {
            // Create default config if not exists
            let current_user = whoami::username();
            let default_kube = KubeConfig{ 
                default_ctx: "simulated".to_string(), 
                clusters: vec!["simulated".to_string()], 
                users: vec![current_user],
            };
            let default = Config { kube: default_kube, simulated: true };
            let toml_str = toml::to_string(&default).expect("Could not serialize default config");
            fs::create_dir_all(path.parent().unwrap())
                .expect("Could not create config directory");
            fs::write(&path, toml_str).expect("Could not write default config");
            default

        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::tempdir;
    use std::env;
    use std::io::Write;
    use std::sync::Mutex;
    use lazy_static::lazy_static;
    use std::panic;
    use std::fs::File;
        
    lazy_static! {
        static ref TEST_MUTEX: Mutex<()> = Mutex::new(());
    }

    fn with_temp_config<F: FnOnce() + Send + 'static>(test: F) {
        let _lock = TEST_MUTEX.lock().unwrap();
        let dir = tempdir().unwrap();
        unsafe {
        env::set_var("XDG_CONFIG_HOME", dir.path());
        }
        test();
        unsafe {
        env::remove_var("XDG_CONFIG_HOME");
        }
    }

    #[test]
    fn test_load_default_config() {
        with_temp_config(|| {
            let config = Config::load();
            assert!(!config.simulated);
        });
    }   
        
        
        
        
    #[test]
    fn test_load_existing_config() {
        with_temp_config(|| {
            let dir = env::var("XDG_CONFIG_HOME").unwrap();
            let config_path = PathBuf::from(dir).join("clust.toml");

            let mut file = File::create(&config_path).unwrap();
            writeln!(file, "simulated = true").unwrap();
            drop(file); // Close the file to ensure it's written

            let config = Config::load();
            assert!(config.simulated);
        });
    }

    #[test]
    fn test_load_invalid_config() {
        with_temp_config(|| {
            let dir = env::var("XDG_CONFIG_HOME").unwrap();
            let config_path = PathBuf::from(dir).join("clust.toml");

            let mut file = File::create(&config_path).unwrap();
            writeln!(file, "invalid_toml").unwrap();
            drop(file); // Close the file to ensure it's written

            let result = panic::catch_unwind(|| {
                Config::load();
            });
            assert!(result.is_err());
        });
    }

    #[test]
    fn test_create_config_directory() {
        with_temp_config(|| {
            let dir = env::var("XDG_CONFIG_HOME").unwrap();
            let config_dir = PathBuf::from(dir);
            fs::remove_dir_all(&config_dir).unwrap_or(()); // Ensure directory does not exist

            let config = Config::load();
            assert!(!config.simulated);
            assert!(config_dir.exists());
        });
    }
}

