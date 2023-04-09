mod lib;

use crate::lib::git::has_remote;
use crate::lib::{
    dir::walk_dirs,
    Git::{Git, NotGit},
};
use std::path::Path;

fn main() {
    let root = "/home/silence/projects/rust";
    let dirs = walk_dirs(Path::new(root));
    dirs.iter().for_each(|dir| match dir {
        Git(path) => {
            if has_remote(path) {
                println!("{} has remote", path.display());
            } else {
                println!("{} dont's has remote", path.display());
            }
        }
        NotGit(path) => {
            println!("{}", path.display());
        }
    })
}
