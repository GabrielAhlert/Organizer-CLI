use std::fs;
use std::path::{Path, PathBuf};
use std::io;

pub enum MoveResult {
    Moved(PathBuf),
    Renamed(PathBuf),
    Ignored(String), // e.g., hidden file or same dir
    Failed(String),
}

pub struct FileOps;

impl FileOps {
    pub fn organize_file(source: &Path, dest_root: &Path, category: &str, ignore_hidden: bool) -> MoveResult {
        // Check if file handles exist
        if !source.exists() || !source.is_file() {
            return MoveResult::Failed("Arquivo de origem inválido".into());
        }

        // Ignore hidden files if requested
        let file_name = match source.file_name() {
            Some(name) => name,
            None => return MoveResult::Failed("Nome de arquivo inválido".into()),
        };

        let name_str = file_name.to_string_lossy();
        if ignore_hidden && name_str.starts_with('.') {
             return MoveResult::Ignored("Arquivo oculto".into());
        }
        
        // Construct destination path
        let dest_folder = dest_root.join(category);
        
        // Create destination folder if it doesn't exist
        if !dest_folder.exists() {
            if let Err(e) = fs::create_dir_all(&dest_folder) {
                return MoveResult::Failed(format!("Falha ao criar diretório: {}", e));
            }
        }

        let mut dest_path = dest_folder.join(file_name);
        let mut is_renamed = false;

        // Prevent overwriting: find a unique name
        if dest_path.exists() {
             // If source and dest are the same file, do nothing
             if let Ok(canon_src) = source.canonicalize() {
                 if let Ok(canon_dest) = dest_path.canonicalize() {
                     if canon_src == canon_dest {
                         return MoveResult::Ignored("Mesmo arquivo (já organizado)".into());
                     }
                 }
             }
             
             // Rename logic: name.ext -> name_1.ext
            let stem = source.file_stem().unwrap_or_default().to_string_lossy();
            let ext = source.extension()
                .map(|e| format!(".{}", e.to_string_lossy()))
                .unwrap_or_default();
            
            let mut counter = 1;
            while dest_path.exists() {
                let new_name = format!("{}_{}{}", stem, counter, ext);
                dest_path = dest_folder.join(new_name);
                counter += 1;
            }
            is_renamed = true;
        }

        // Perform move
        match fs::rename(source, &dest_path) {
            Ok(_) => {
                if is_renamed {
                    MoveResult::Renamed(dest_path)
                } else {
                    MoveResult::Moved(dest_path)
                }
            },
            Err(e) => MoveResult::Failed(format!("Erro ao mover: {}", e)),
        }
    }
}
