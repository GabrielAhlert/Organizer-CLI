use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use directories::ProjectDirs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub rules: HashMap<String, Vec<String>>,
}

impl Default for Config {
    fn default() -> Self {
        let mut rules = HashMap::new();
        
        rules.insert("Imagens".to_string(), vec![
            "jpg", "jpeg", "png", "gif", "svg", "webp", "bmp", "tiff"
        ].into_iter().map(String::from).collect());

        rules.insert("Documentos".to_string(), vec![
            "pdf", "docx", "doc", "txt", "xlsx", "pptx", "md", "csv", "epub"
        ].into_iter().map(String::from).collect());

        rules.insert("Videos".to_string(), vec![
            "mp4", "mov", "avi", "mkv", "webm", "flv", "wmv"
        ].into_iter().map(String::from).collect());

        rules.insert("Audio".to_string(), vec![
            "mp3", "wav", "flac", "ogg", "aac", "m4a"
        ].into_iter().map(String::from).collect());

        rules.insert("Compactados".to_string(), vec![
            "zip", "rar", "tar", "gz", "7z", "bz2"
        ].into_iter().map(String::from).collect());

        rules.insert("Aplicativos".to_string(), vec![
            "exe", "msi", "dmg", "deb", "rpm", "bat", "sh"
        ].into_iter().map(String::from).collect());

        rules.insert("Codigos".to_string(), vec![
            "rs", "py", "js", "ts", "html", "css", "c", "cpp", "java", "go"
        ].into_iter().map(String::from).collect());

        Self { rules }
    }
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path()?;

        if !config_path.exists() {
            let default_config = Config::default();
            default_config.save(&config_path)?;
            return Ok(default_config);
        }

        let content = fs::read_to_string(&config_path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let proj_dirs = ProjectDirs::from("", "", "organizer")
            .ok_or("Could not determine config directory")?;
        
        Ok(proj_dirs.config_dir().join("config.toml"))
    }

    pub fn open_in_editor() -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::get_config_path()?;
        
        // Ensure it exists before opening
        if !path.exists() {
             let default_config = Config::default();
             default_config.save(&path)?;
        }

        println!("Abrindo editor para: {:?}", path);
        open::that(path)?;
        Ok(())
    }
}
