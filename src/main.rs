use home::home_dir;
use std::process;
use tokio::task::JoinSet;

struct SelectedDir {
    path: String,
    size: u64,
    root_name: String,
}

#[tokio::main]
async fn main() {
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

    let folders_to_scan = vec!["node_modules", "target"]
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let mut selected_dirs: Vec<SelectedDir> = Vec::new();
    let (tx, mut rx) = tokio::sync::mpsc::channel::<SelectedDir>(100);
}
