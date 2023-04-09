use std::path::Path;

use crate::lib::dir::is_remote_local_syuc;
use crate::lib::git::has_remote;
use crate::lib::{
    dir::walk_dirs,
    Git::{Git, NotGit},
};

mod lib;

fn main() {
    let root = "/home/silence/projects/rust";
    let dirs = walk_dirs(Path::new(root));
    dirs.iter().for_each(|dir| match dir {
        Git(path) => {
            if has_remote(path) {
                let same = is_remote_local_syuc(path);
                if !same {
                    // TODO: 以Orange字体输出
                    println!("remote and local are different: {:?}", path);
                } else {
                    // TODO: 以Green字体输出
                    println!("remote and local are same: {:?}", path);
                }
            } else {
                // TODO: 以黄色字体输出
                println!("{:?} dont's has remote", path);
            }
        }
        NotGit(path) => {
            // TODO: 以红色字体输出
            println!("{:?}", path);
        }
    })
}
