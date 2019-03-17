use std::fs::{metadata, read_to_string};
use std::path::PathBuf;
use std::process::exit;

use walkdir::WalkDir;

pub struct VoidWalk {
    file_paths: Vec<PathBuf>,
}

/// Ensure that the root directory exists and is possible to access.
fn check_root_dir(root: &str) {
    match metadata(root) {
        Ok(m) => {
            if !m.is_dir() {
                error!("{} is not a directory!", root);
                exit(2);
            }
        }
        Err(e) => {
            error!("Failed to read '{}': {}", root, e);
            exit(2);
        }
    };
}

impl VoidWalk {
    /// Create a new VoidWalk object which recursively finds all .void
    /// files within the specified root folder.
    pub fn new(root: &str) -> Self {
        check_root_dir(root);

        info!("Scanning directory '{}' for void files...", root);

        let walk_dir = WalkDir::new(root).into_iter();

        let files = walk_dir
            .into_iter()
            .filter_map(|d| match d {
                Ok(d) => Some(d.path().to_path_buf()),
                Err(e) => {
                    error!("{}", e);
                    exit(2);
                }
            })
            .filter(|p| p.extension() != None && p.extension().unwrap() == "void")
            .collect::<Vec<PathBuf>>();

        info!("Found {:?} void files", files.len());

        VoidWalk { file_paths: files }
    }

    /// Process all .void files found in the root directory.
    pub fn read_files(&self) {
        for (i, fp) in (&self).file_paths.iter().enumerate() {
            info!(
                "[{}/{}] Processing {}",
                i + 1,
                &self.file_paths.len(),
                fp.display()
            );

            match read_to_string(fp) {
                Err(e) => warn!("Failed to process '{}': {}", fp.display(), e),
                Ok(s) => trace!("File: {}\n---\n{}\n---", fp.display(), s),
            }
        }
    }
}
