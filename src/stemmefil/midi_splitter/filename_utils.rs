use std::path::{Path, PathBuf};

const VACANT_FILENAME_ATTEMPTS_LIMIT: usize = 10;

pub fn find_vacant_filename<S: AsRef<str>, P: AsRef<Path>>(out_dir: P, file_stem: S, extension: S) -> (String, PathBuf) {
    let ideal_filename = format!("{}.{}", file_stem.as_ref(), extension.as_ref());
    let ideal_path = out_dir.as_ref().join(&ideal_filename);
    if !ideal_path.exists() {
        (ideal_filename, ideal_path)
    } else {
        for i in 1..=VACANT_FILENAME_ATTEMPTS_LIMIT {
            let next_filename = format!("{} ({}).{}", file_stem.as_ref(), i, extension.as_ref());
            let next_path = ideal_path.with_file_name(&next_filename);
            if !next_path.exists() {
                return (next_filename, next_path);
            }
        }
        eprintln!("Gave up finding vacant filename.");
        (ideal_filename, ideal_path)
    }
}