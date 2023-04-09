use crate::lib::Git;
use std::fs;
use std::path::Path;

pub fn walk_dirs(dir: &Path) -> Vec<Git> {
    fs::read_dir(dir)
        .unwrap()
        .flat_map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                if dir_has_git_dir(&path) {
                    return vec![Git::Git(path.into_boxed_path())];
                }
                let mut v = crate::walk_dirs(&path);
                v.push(Git::NotGit(path.into_boxed_path()));
                return v;
            }
            vec![]
        })
        .collect()
}
/// 该函数检查目录是否包含“.git”目录。
///
/// 参数:
///
/// * `dir`: `dir` 参数是对 `Path` 对象的引用，表示文件系统路径。函数 `dir_has_git_dir` 将此路径作为输入并检查它是否包含名为“.git”的目录。
///
/// 返回得::
///
/// 一个布尔值，指示给定目录是否包含名为“.git”的子目录。
fn dir_has_git_dir(dir: &Path) -> bool {
    let dirs = fs::read_dir(dir);
    match dirs {
        Ok(mut dirs) => {
            dirs.any(|item| item.map_or_else(|_| false, |entry| entry.path().ends_with(".git")))
        }
        Err(err) => {
            println!("{}", err);
            false
        }
    }
}

pub fn read_file(file: &Path) -> std::io::Result<String> {
    fs::read_to_string(file)
}
