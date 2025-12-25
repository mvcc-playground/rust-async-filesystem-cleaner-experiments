use jwalk::WalkDir;

fn scan_folders_parallel(path: &str) -> std::io::Result<()> {
    WalkDir::new(path)
        .skip_hidden(false)
        .parallelism(jwalk::Parallelism::RayonNewPool(100))
        .process_read_dir(|_, _, _, children| {
            // Filtra ANTES de processar os filhos
            children.retain(|dir_entry_result| {
                dir_entry_result
                    .as_ref()
                    .map(|dir_entry| {
                        let file_name = dir_entry.file_name.to_string_lossy();
                        file_name != ".git"
                            && file_name != "node_modules"
                            && file_name != "target"
                            && file_name != ".expo"
                    })
                    .unwrap_or(false)
            });
        })
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_dir())
        .for_each(|entry| {
            println!("{}, true", entry.path().display());
        });

    Ok(())
}

fn main() -> std::io::Result<()> {
    let start = std::time::Instant::now();
    println!("Start time: {:?}", start);

    scan_folders_parallel(".")?;

    let duration = start.elapsed();
    println!("\nDuration: {:?}", duration);
    Ok(())
}
