use std::path::Path;

pub fn has_remote(dir: &Path) -> bool {
    let dir = dir.join(".git/refs/remotes");
    dir.is_dir()
}
