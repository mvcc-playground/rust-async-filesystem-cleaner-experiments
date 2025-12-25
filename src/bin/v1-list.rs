use home::home_dir;
use std::{
    fs,
    path::{Path, PathBuf},
    process,
};

fn main() {
    let home_dir = home_dir().expect("Failed to get home directory");
    let current_dir = std::env::current_dir().expect("Failed to get current directory");

    if current_dir == home_dir {
        eprintln!("Cannot run in the home directory");
        process::exit(1);
    }

    if !current_dir.starts_with(&home_dir) {
        eprintln!("Current directory is not inside home directory");
        process::exit(1);
    }

    let folders_to_scan = vec!["node_modules", "target"];

    let stats =
        calculate_folders(&current_dir, &folders_to_scan).expect("Failed to calculate folders");

    println!("\n=== Resumo ===");
    println!(
        "Total size: {:.2} MB",
        stats.total_size as f64 / 1_048_576.0
    );
    println!("Total count: {}", stats.folders.len());

    println!("\n=== Pastas encontradas ===");
    for folder in &stats.folders {
        println!(
            "{} - {:.2} MB",
            folder.path.display(),
            folder.size as f64 / 1_048_576.0
        );
    }

    // Exemplo: encontrar a maior pasta
    if let Some(largest) = stats.folders.iter().max_by_key(|f| f.size) {
        println!("\n=== Maior pasta ===");
        println!(
            "{} - {:.2} MB",
            largest.path.display(),
            largest.size as f64 / 1_048_576.0
        );
    }
}

// Struct para armazenar informações de cada pasta
#[derive(Debug, Clone)]
struct FolderInfo {
    path: PathBuf,
    size: u64,
}

// Struct para armazenar estatísticas gerais
struct DirStats {
    folders: Vec<FolderInfo>,
    total_size: u64,
}

fn calculate_folders(dir: &Path, folders_to_scan: &[&str]) -> std::io::Result<DirStats> {
    let mut stats = DirStats {
        folders: Vec::new(),
        total_size: 0,
    };

    if dir.is_dir() {
        let entries = fs::read_dir(dir)?;

        for entry in entries {
            let path = entry?.path();

            if path.is_dir() {
                // Verifica se é uma pasta que queremos escanear
                if folders_to_scan
                    .iter()
                    .any(|&f| path.file_name().map_or(false, |name| name == f))
                {
                    let dir_size = calculate_dir_size(&path)?;

                    // Salva informações da pasta em memória
                    stats.folders.push(FolderInfo {
                        path: path.clone(),
                        size: dir_size,
                    });

                    stats.total_size += dir_size;

                    println!(
                        "Found: {} ({:.2} MB)",
                        path.display(),
                        dir_size as f64 / 1_048_576.0
                    );
                } else {
                    // Recursão: busca em subdiretórios
                    let sub_stats = calculate_folders(&path, folders_to_scan)?;
                    stats.folders.extend(sub_stats.folders);
                    stats.total_size += sub_stats.total_size;
                }
            }
        }
    }

    Ok(stats)
}

fn calculate_dir_size(dir: &Path) -> std::io::Result<u64> {
    let mut total_size = 0;

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            total_size += fs::metadata(&path)?.len();
        } else if path.is_dir() {
            total_size += calculate_dir_size(&path)?;
        }
    }

    Ok(total_size)
}
