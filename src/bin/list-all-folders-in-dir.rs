use std::fs;

fn scan_folders(path: &str) -> std::io::Result<()> {
    let entries = fs::read_dir(path)?;

    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();

        println!(
            "{}, {}",
            entry.path().display(),
            entry.file_type()?.is_dir()
        );

        // Compara apenas o NOME do diretório, não o caminho completo
        if entry.file_type()?.is_dir()
            && file_name_str != ".git"
            && file_name_str != "node_modules"
            && file_name_str != "target"
            && file_name_str != ".expo"
        {
            // Passa o caminho do subdiretório para a recursão
            scan_folders(&entry.path().display().to_string())?;
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    // start timer
    let start = std::time::Instant::now();
    println!("Start time: {:?}", start);
    scan_folders(".")?;
    let duration = start.elapsed();
    println!("Duration: {:?}", duration);
    Ok(())
}
