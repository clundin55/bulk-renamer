use std::fs::rename;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

pub type RenameError = Box<dyn std::error::Error>;

pub fn bulk_rename(
    root_dir: &Path,
    existing_name: &str,
    new_name: &str,
) -> Result<Vec<String>, RenameError> {
    let matches = find_matching_entries(existing_name, root_dir)?;
    let operations = matches
        .iter()
        .filter_map(|m| {
            if let Some(new_name) = rename_file(m, existing_name, new_name) {
                return Some(new_name);
            }
            None
        })
        .collect();

    Ok(operations)
}

fn find_matching_entries(
    existing_name: &str,
    root_dir: &Path,
) -> Result<Vec<PathBuf>, RenameError> {
    let w = WalkDir::new(root_dir);

    let matching_files = w.into_iter().filter_map(|e| match e {
        Ok(entry) if is_match(existing_name, &entry) => Some(entry.into_path()),
        _ => None,
    });

    Ok(matching_files.collect())
}

fn rename_file(file: &Path, existing_name: &str, new_name: &str) -> Option<String> {
    if let Some(file_name) = file.to_str() {
        let new_name = file_name.replace(existing_name, new_name);

        if rename(file_name, &new_name).is_ok() {
            return Some(new_name);
        }
    }
    None
}

fn is_match(target: &str, entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.contains(target))
        .unwrap_or(false)
}
