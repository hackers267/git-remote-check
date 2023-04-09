use crate::lib::dir::read_file;
use std::fs;
use std::path::Path;

pub fn has_remote(dir: &Path) -> bool {
    let dir = dir.join(".git/refs/remotes");
    dir.is_dir()
}

pub fn remote_files(dir: &Path) -> Vec<Box<Path>> {
    let dir = dir.join(".git/refs/remotes");
    get_files(&dir)
}

pub fn remote_file_contents(dir: &Path) -> Vec<std::io::Result<String>> {
    let files = remote_files(dir);
    file_contents(&files)
}
pub fn local_file_contents(dir: &Path) -> Vec<std::io::Result<String>> {
    let files = local_files(dir);
    file_contents(&files)
}

fn file_contents(files: &[Box<Path>]) -> Vec<std::io::Result<String>> {
    files
        .iter()
        .map(|file| {
            let contents = read_file(file);
            return contents;
        })
        .collect()
}

pub fn local_files(dir: &Path) -> Vec<Box<Path>> {
    let dir = dir.join(".git/refs/heads");
    get_files(&dir)
}

fn get_files(dir: &Path) -> Vec<Box<Path>> {
    if dir.is_dir() {
        let r: Vec<_> = fs::read_dir(dir)
            .unwrap()
            .flat_map(|item| {
                let path = item.unwrap().path();
                if path.is_dir() {
                    get_files(&path)
                } else if is_head_file(&path) {
                    vec![]
                } else {
                    vec![path.into_boxed_path()]
                }
            })
            .collect();
        return r;
    }
    vec![dir.to_owned().into_boxed_path()]
}

fn is_head_file(path: &Path) -> bool {
    path.ends_with("HEAD")
}
