use walkdir::WalkDir;
#[cfg(target_os = "windows")]
use sysinfo::Disks;

use crate::algorithim::{OutputPath, OutputJson};


pub async fn search_file_by_name(root: String, search_term: String) -> Option<OutputJson> {
    let mut found_paths: Vec<OutputPath> = Vec::new();
    let mut i: i32 = 0;
    for file in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        println!("searched file: {}", file.path().display());
        if let Some(name) = file.file_name().to_str() {
            if name.contains(&search_term) {
                let found_path = OutputPath::new(
                    file.path().display().to_string(),
                    i
                );
                found_paths.push(found_path);
                i += 1;

            }


        }
    }
    Some(OutputJson::new(found_paths))
}

pub async fn search_file_by_ext(root: String, ext: Option<String>) -> Option<OutputJson> {
    let mut found_paths: Vec<OutputPath> = Vec::new();
    let mut i: i32 = 0;
    for file in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        println!("searched file: {}", file.path().display());
        if let Some(file_ext) = file.path().extension().and_then(|e| e.to_str()) {
            if file_ext == ext.clone().unwrap() {
                let found_path = OutputPath::new(
                    file.path().display().to_string(),
                    i
                );
                found_paths.push(found_path);
                i += 1;
            }

        }
    }
    Some(OutputJson::new(found_paths))
}

pub async fn search_file_by_both(root: String, search_term: Option<String>, ext: Option<String>) -> Option<OutputJson> {
    let mut found_paths: Vec<OutputPath> = Vec::new();
    let mut i: i32 = 0;
    for file in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        if let Some(name) = file.file_name().to_str() {
            println!("searched file: {}", file.path().display());
            if name.contains(&search_term.clone().unwrap()) {
                if let Some(file_ext) = file.path().extension().and_then(|e| e.to_str()) {
                    if file_ext == ext.clone().unwrap() {
                        let found_path = OutputPath::new(
                            file.path().display().to_string(),
                            i
                        );
                        found_paths.push(found_path);
                        i += 1;
                    }
                }

            }
        }
    }
    Some(OutputJson::new(found_paths))
}

#[cfg(target_os = "windows")]
fn get_drive_letters() -> Vec<String> {
    let disks = Disks::new_with_refreshed_list();

    disks.iter()
        .map(|d| d.mount_point().to_str().unwrap_or("").to_string())
    .collect()
}

#[cfg(target_os = "windows")]
pub fn get_roots() -> Vec<String> {
    let roots: Vec<String> = get_drive_letters();
    for (i, root) in roots.iter().enumerate() {
        if root.contains("C") {

            roots.clone()[i] = format!("C:/Users/{:?}", whoami::username());
        }
    }
    roots
}
