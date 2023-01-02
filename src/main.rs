use std::{
    collections::HashMap,
    env::current_dir,
    fs::read_dir,
    path::{Path, PathBuf},
};

use probe_rs::config::{add_target_from_yaml, families};

fn main() {
    let subset = vec!["MKL17Z4", "MIMXRT685S", "MIMXRT1166"];

    println!("BEFORE: {:#?}", filter_families(subset.as_slice()));

    let mut path = current_dir().unwrap();
    path.push("targets");

    let targets = get_targets(&path);

    for target in targets {
        if let Err(err) = add_target_from_yaml(target.as_path()) {
            println!(
                "Failed to add targets in file {}: {}",
                target.display(),
                err
            )
        }
    }

    println!("AFTER: {:#?}", filter_families(subset.as_slice()));
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

fn filter_families(subset: &[&str]) -> HashMap<String, Vec<String>> {
    HashMap::from_iter(families().unwrap().into_iter().filter_map(|family| {
        if subset.contains(&family.name.as_str()) {
            Some((
                family.name.clone(),
                family
                    .variants
                    .into_iter()
                    .map(|chip| chip.name)
                    .collect::<Vec<_>>(),
            ))
        } else {
            None
        }
    }))
}
