mod lib;

use crate::lib::git::{
    has_remote, local_file_contents, local_files, remote_file_contents, remote_files,
};
use crate::lib::{
    dir::walk_dirs,
    Git::{Git, NotGit},
};
use std::collections::HashSet;
use std::path::Path;

fn main() {
    let root = "/home/silence/projects/rust";
    let dirs = walk_dirs(Path::new(root));
    dirs.iter().for_each(|dir| match dir {
        Git(path) => {
            if has_remote(path) {
                let remote_contents = remote_file_contents(path);
                let local_contents = local_file_contents(path);
                let remote: HashSet<_> = remote_contents
                    .iter()
                    .filter_map(|item| item.as_ref().ok())
                    .collect();
                let local: HashSet<_> = local_contents
                    .iter()
                    .filter_map(|item| item.as_ref().ok())
                    .collect();
                let same = !remote.is_disjoint(&local);
                if !same {
                    println!("remote and local are different: {:?}", path);
                }
            } else {
                println!("{} dont's has remote", path.display());
            }
        }
        NotGit(path) => {
            println!("{}", path.display());
        }
    })
}
