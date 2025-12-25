use std::fs;
use tokio::task::JoinSet;

#[derive(Clone)]
struct FilterConfig {
    show_names: Vec<String>,
    show_extensions: Vec<String>,
    skip_dirs: Vec<String>,
}

impl FilterConfig {
    fn should_show(&self, file_name: &str, is_dir: bool) -> bool {
        // Verifica nome exato
        if self.show_names.iter().any(|n| n == file_name) {
            return true;
        }

        // Verifica extensão para arquivos
        if !is_dir {
            if let Some(pos) = file_name.rfind('.') {
                let ext = &file_name[pos + 1..];
                if self.show_extensions.iter().any(|e| e == ext) {
                    return true;
                }
            }
        }

        false
    }

    fn should_skip(&self, dir_name: &str) -> bool {
        self.skip_dirs.iter().any(|d| d == dir_name)
    }
}

async fn scan_folders_concurrent(initial_path: &str, config: FilterConfig) -> std::io::Result<()> {
    let mut tasks = JoinSet::new();
    let mut pending_dirs = vec![initial_path.to_string()];

    while !pending_dirs.is_empty() || !tasks.is_empty() {
        while tasks.len() < 1000 && !pending_dirs.is_empty() {
            let path = pending_dirs.pop().unwrap();
            let config_clone = config.clone();

            tasks.spawn_blocking(move || {
                let mut new_dirs = Vec::new();
                let mut matches = Vec::new();

                if let Ok(entries) = fs::read_dir(&path) {
                    for entry in entries.flatten() {
                        if let Ok(file_type) = entry.file_type() {
                            let entry_path = entry.path();
                            let file_name = entry.file_name();
                            let file_name_str = file_name.to_string_lossy();
                            let is_dir = file_type.is_dir();

                            // Verifica se deve mostrar
                            if config_clone.should_show(&file_name_str, is_dir) {
                                matches.push(entry_path.display().to_string());
                            }

                            // Se for diretório e não está na lista skip, entra nele
                            if is_dir && !config_clone.should_skip(&file_name_str) {
                                new_dirs.push(entry_path.display().to_string());
                            }
                        }
                    }
                }
                (new_dirs, matches)
            });
        }

        if let Some(result) = tasks.join_next().await {
            if let Ok((new_dirs, matches)) = result {
                pending_dirs.extend(new_dirs);
                for m in matches {
                    println!("{}", m);
                }
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let start = std::time::Instant::now();
    println!("Start time: {:?}", start);

    let config = FilterConfig {
        // Nomes exatos para mostrar
        show_names: vec!["node_modules".to_string(), "target".to_string()],
        // Extensões para mostrar (sem o ponto)
        show_extensions: vec!["log".to_string()],
        // Diretórios para NÃO ENTRAR (mas podem ser mostrados se estiverem em show_names)
        skip_dirs: vec![
            ".git".to_string(),
            "node_modules".to_string(),
            "target".to_string(),
            ".expo".to_string(),
        ],
    };

    scan_folders_concurrent(".", config).await?;

    let duration = start.elapsed();
    println!("\nDuration: {:?}", duration);
    Ok(())
}
