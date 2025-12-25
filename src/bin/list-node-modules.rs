use std::fs;
use tokio::task::JoinSet;

async fn scan_folders_concurrent(initial_path: &str) -> std::io::Result<()> {
    let mut tasks = JoinSet::new();
    let mut pending_dirs = vec![initial_path.to_string()];

    while !pending_dirs.is_empty() || !tasks.is_empty() {
        while tasks.len() < 1000 && !pending_dirs.is_empty() {
            let path = pending_dirs.pop().unwrap();

            tasks.spawn_blocking(move || {
                let mut new_dirs = Vec::new();

                if let Ok(entries) = fs::read_dir(&path) {
                    for entry in entries.flatten() {
                        if let Ok(file_type) = entry.file_type() {
                            let file_name = entry.file_name();
                            let file_name_str = file_name.to_string_lossy();

                            if file_type.is_dir() {
                                if file_name_str == "node_modules" {
                                    println!("{}", entry.path().display());
                                } else if file_name_str != ".git"
                                    && file_name_str != "node_modules"
                                    && file_name_str != "target"
                                    && file_name_str != ".expo"
                                {
                                    new_dirs.push(entry.path().display().to_string());
                                }
                            }
                        }
                    }
                }
                new_dirs
            });
        }

        if let Some(result) = tasks.join_next().await {
            if let Ok(new_dirs) = result {
                pending_dirs.extend(new_dirs);
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let start = std::time::Instant::now();
    println!("Start time: {:?}", start);

    scan_folders_concurrent(".").await?;

    let duration = start.elapsed();
    println!("\nDuration: {:?}", duration);
    Ok(())
}
