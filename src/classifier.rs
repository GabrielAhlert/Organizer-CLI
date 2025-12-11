use std::path::Path;
use crate::config::Config;

pub struct FileClassifier;

impl FileClassifier {
    /// Determines the category folder for a given file based on its extension and configuration.
    /// Returns "Outros" if no match is found.
    pub fn classify(path: &Path, config: &Config) -> String {
        // Handle files without extension
        let extension = match path.extension() {
            Some(ext) => ext.to_string_lossy().to_lowercase(),
            None => return "Outros".to_string(),
        };

        // Check against rules in config
        for (category, extensions) in &config.rules {
            // Case-insensitive comparison (extensions in config should be handled as lowercase or check both)
            if extensions.iter().any(|ext| ext.to_lowercase() == extension) {
                return category.clone();
            }
        }

        "Outros".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_classify_image() {
        let config = Config::default();
        let path = Path::new("foto.jpg");
        assert_eq!(FileClassifier::classify(path, &config), "Imagens");
    }

    #[test]
    fn test_classify_unknown() {
        let config = Config::default();
        let path = Path::new("arquivo.xyz123");
        assert_eq!(FileClassifier::classify(path, &config), "Outros");
    }

    #[test]
    fn test_classify_no_extension() {
        let config = Config::default();
        let path = Path::new("README");
        assert_eq!(FileClassifier::classify(path, &config), "Outros");
    }
}
