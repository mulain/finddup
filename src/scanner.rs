use crate::models::{DuplicateGroup, FileInfo};
use itertools::Itertools;
use rayon::prelude::*;
use sha2::{Digest, Sha256};
use std::{collections::HashMap, fs::File, io::Read, path::Path};
use walkdir::WalkDir;

pub fn find_duplicates(path: &Path, min_size: u64) -> Vec<DuplicateGroup> {
    let size_map: HashMap<u64, Vec<FileInfo>> = get_size_map(path, min_size);

    let mut duplicates: Vec<DuplicateGroup> = Vec::new();

    for (_size, group) in size_map {
        if group.len() < 2 {
            continue;
        }

        let hash_map: HashMap<String, Vec<std::path::PathBuf>> = group
            .into_par_iter()
            .filter_map(|mut file| {
                compute_hash(&file.path).map(|hash| {
                    file.hash = Some(hash.clone());
                    (hash, file.path)
                })
            })
            .fold(
                || HashMap::<String, Vec<std::path::PathBuf>>::new(),
                |mut acc, (hash, path)| {
                    acc.entry(hash).or_default().push(path);
                    acc
                },
            )
            .reduce(
                || HashMap::<String, Vec<std::path::PathBuf>>::new(),
                |mut acc, map| {
                    for (hash, paths) in map {
                        acc.entry(hash).or_default().extend(paths);
                    }
                    acc
                },
            );

        for (hash, paths) in hash_map {
            if paths.len() > 1 {
                duplicates.push(DuplicateGroup { hash, files: paths });
            }
        }
    }

    duplicates
}

fn get_size_map(path: &Path, min_size: u64) -> HashMap<u64, Vec<FileInfo>> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| {
            let metadata = e.metadata().ok()?;
            if metadata.len() < min_size {
                return None;
            }
            Some(FileInfo {
                path: e.path().to_path_buf(),
                size: metadata.len(),
                hash: None, // to be computed later
            })
        })
        .into_group_map_by(|file| file.size)
}

fn compute_hash(path: &std::path::Path) -> Option<String> {
    let mut file = File::open(path).ok()?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];

    loop {
        let n = file.read(&mut buffer).ok()?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    Some(format!("{:x}", hasher.finalize()))
}
