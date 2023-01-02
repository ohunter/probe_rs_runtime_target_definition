use std::{
    env::current_dir,
    fs::read_dir,
    path::{Path, PathBuf},
};

use probe_rs::config::add_target_from_yaml;

fn main() {
    let mut path = current_dir().unwrap();
    path.push("targets");

    let targets = get_targets(&path);

    for target in targets {
        match add_target_from_yaml(target.as_path()) {
            Ok(()) => println!(
                "Targets in file {} were added successfully",
                target.display()
            ),
            Err(err) => println!(
                "Failed to add targets in file {}: {}",
                target.display(),
                err
            ),
        }
    }
}

fn get_targets(path: &Path) -> Vec<PathBuf> {
    read_dir(path)
        .unwrap()
        .into_iter()
        .filter_map(|item| {
            if let Ok(item) = item {
                match item.path().extension() {
                    None => None,
                    Some(os_str) => match os_str.to_str() {
                        Some("yaml") | Some("yml") => Some(item.path()),
                        Some(&_) | None => None,
                    },
                }
            } else {
                None
            }
        })
        .collect()
}
