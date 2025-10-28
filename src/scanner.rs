use crate::models::{DuplicateGroup, FileInfo};
use rayon::prelude::*;
use sha2::{Digest, Sha256};
use std::{collections::HashMap, fs::File, io::Read, path::Path};
use walkdir::WalkDir;

pub fn find_duplicates(path: &Path, min_size: u64, threads: usize) -> Vec<DuplicateGroup> {
  /*   if threads > 0 {
        rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build_global()
            .ok();
    } */

    let pool = rayon::ThreadPoolBuilder::new()
    .num_threads(threads.max(1))
    .build()
    .expect("Failed to build thread pool");

    let mut files: Vec<FileInfo> = WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| entry.ok())
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
        .collect();

    let mut size_map: HashMap<u64, Vec<FileInfo>> = HashMap::new();
    for file in files.drain(..) {
        size_map.entry(file.size).or_default().push(file);
    }

    let mut duplicates: Vec<DuplicateGroup> = Vec::new();

    for (_size, group) in size_map {
        if group.len() < 2 {
            continue;
        }

        // Compute hashes in parallel for files of the same size
        /*let mut hash_map: HashMap<String, Vec<std::path::PathBuf>> = HashMap::new();
        
         group.into_par_iter()
            .filter_map(|file| compute_hash(&file.path).map(|hash| (hash, file.path)))
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|(hash, path)| {
                hash_map.entry(hash).or_default().push(path);
            });
 */

 let hash_map: HashMap<String, Vec<std::path::PathBuf>> = pool.install(|| {
    group
        .into_par_iter()
        .filter_map(|mut file| {
            compute_hash(&file.path).map(|hash| {
                file.hash = Some(hash.clone()); // optional: store in FileInfo
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
        )
});


        // Add groups with duplicates to results
        for (hash, paths) in hash_map {
            if paths.len() > 1 {
                duplicates.push(DuplicateGroup { hash, files: paths });
            }
        }
    }

    duplicates
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