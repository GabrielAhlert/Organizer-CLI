use clap::{Parser, Subcommand};
use colored::*;
use std::path::{Path, PathBuf};
use std::fs;
use std::time::Instant;

mod config;
mod classifier;
mod ops;

use config::Config;
use classifier::FileClassifier;
use ops::{FileOps, MoveResult};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Diretório a ser organizado (default: atual)
    #[arg(default_value = ".")]
    input_dir: PathBuf,

    /// Diretório de destino (default: mesmo que input)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Ignorar arquivos ocultos
    #[arg(long)]
    ignore_hidden: bool,

    /// Executar recursivamente (não implementado nesta versão, mas preparado)
    #[arg(short, long)]
    recursive: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Abre o arquivo de configuração e regras
    Config,
}

fn main() {
    let cli = Cli::parse();

    // Handle 'config' subcommand
    if let Some(Commands::Config) = cli.command {
        if let Err(e) = Config::open_in_editor() {
            eprintln!("{} {}", "[ERRO]".red().bold(), e);
        }
        return;
    }

    // Main organization flow
    let start_time = Instant::now();
    
    // Resolve paths
    let input_dir = cli.input_dir;
    let output_dir = cli.output.clone().unwrap_or_else(|| input_dir.clone());

    if !input_dir.exists() || !input_dir.is_dir() {
        eprintln!("{} O diretório '{:?}' não existe ou não é válido.", "[ERRO FATAL]".red().bold(), input_dir);
        std::process::exit(1);
    }

    // Load configuration
    let config = match Config::load() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("{} Erro ao carregar config: {}. Usando defaults.", "[AVISO]".yellow(), e);
            Config::default()
        }
    };

    println!("{} Organizando '{:?}' -> '{:?}'", "[INFO]".blue().bold(), input_dir, output_dir);

    // Collect files (non-recursive for now)
    let entries = match fs::read_dir(&input_dir) {
        Ok(entries) => entries,
        Err(e) => {
             eprintln!("{} Falha ao ler diretório: {}", "[ERRO]".red(), e);
             std::process::exit(1);
        }
    };

    let mut moved_count = 0;
    let mut collision_count = 0;
    let mut error_count = 0;

    for entry in entries.flatten() {
        let path = entry.path();
        
        // Skip directories and the output directory itself to avoid recursion loops if configured that way
        if path.is_dir() {
            continue;
        }

        // Don't move the executable itself or config file in current dir
        if let Some(name) = path.file_name() {
             let name_str = name.to_string_lossy();
             if name_str == "organizer.exe" || name_str == "organizer" || name_str == "config.toml" {
                 continue;
             }
        }

        let category = FileClassifier::classify(&path, &config);
        
        // If "Outros" and we might want to skip logic if configured (requirement said map extensions, usually implied others are moved too)
        // Here we move everything.

        let result = FileOps::organize_file(&path, &output_dir, &category, cli.ignore_hidden);

        match result {
            MoveResult::Moved(new_path) => {
                println!("{} {:?} -> {:?}", "[MOVIDO]".green(), path.file_name().unwrap(), new_path);
                moved_count += 1;
            },
            MoveResult::Renamed(new_path) => {
                 println!("{} {:?} -> {:?} (Renomeado)", "[RENOMEADO]".yellow(), path.file_name().unwrap(), new_path);
                 moved_count += 1;
                 collision_count += 1;
            },
            MoveResult::Ignored(_) => {
                // Verbose mode only?
            },
            MoveResult::Failed(err) => {
                eprintln!("{} Falha ao mover {:?}: {}", "[ERRO]".red(), path.file_name().unwrap(), err);
                error_count += 1;
            }
        }
    }

    let duration = start_time.elapsed();
    
    println!("\n{}", "Resumo da Organização:".bold().underline());
    println!("Arquivos Movidos: {}", moved_count);
    println!("Conflitos resolvidos: {}", collision_count);
    println!("Erros: {}", error_count);
    println!("Tempo: {:.2?}", duration);
}
