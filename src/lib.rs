use std::fs::rename;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

#[derive(PartialEq)]
pub enum Operation {
    Mutating,
    DryRun,
}

pub struct FileRename(String, String);

impl FileRename {
    pub fn new(original_name: String, updated_name: String) -> Self {
        Self(original_name, updated_name)
    }
    pub fn get_original_name(&self) -> &str {
        &self.0
    }

    pub fn get_updated_name(&self) -> &str {
        &self.1
    }
}

pub type RenameError = Box<dyn std::error::Error>;

pub fn bulk_rename(
    root_dir: &Path,
    original: &str,
    update: &str,
    operation: &Operation,
) -> Result<Vec<FileRename>, RenameError> {
    let matches = find_matching_entries(original, root_dir)?;
    let operations: Vec<FileRename> = matches
        .iter()
        .filter_map(|m| file_rename(m, original, update))
        .collect();

    for names in &operations {
        let res = match operation {
            Operation::Mutating => rename(names.get_original_name(), names.get_updated_name()),
            Operation::DryRun => Ok(()),
        };
        if let Err(e) = res {
            eprintln!(
                "Failed to rename {} due to {}",
                names.get_original_name(),
                e
            );
        }
    }

    Ok(operations)
}

fn find_matching_entries(
    existing_name: &str,
    root_dir: &Path,
) -> Result<Vec<PathBuf>, RenameError> {
    let w = WalkDir::new(root_dir);

    let matching_files = w.into_iter().filter_map(|e| match e {
        Ok(entry) if matches_existing_name(existing_name, &entry) => Some(entry.into_path()),
        _ => None,
    });

    Ok(matching_files.collect())
}

fn file_rename(file: &Path, original_name: &str, new_name: &str) -> Option<FileRename> {
    if let Some(file_name) = file.to_str() {
        let new_file_name = file_name.replace(original_name, new_name);
        return Some(FileRename::new(file_name.to_owned(), new_file_name));
    }
    None
}

fn matches_existing_name(target: &str, entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.contains(target))
        .unwrap_or(false)
}
