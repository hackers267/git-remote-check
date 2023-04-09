use crate::lib::dir::is_remote_local_syuc;
use crate::lib::git::has_remote;
use crate::lib::{
    dir::walk_dirs,
    Git::{Git, NotGit},
};
use colored::Colorize;
use std::path::Path;

mod lib;

fn main() {
    let root = "/home/silence/projects/rust";
    let dirs = walk_dirs(Path::new(root));
    dirs.iter().for_each(|dir| match dir {
        Git(path) => {
            if has_remote(path) {
                let same = is_remote_local_syuc(path);
                if !same {
                    let info = format!("remote and local are different: {:?}", path);
                    println!("{}", info.magenta());
                } else {
                    let msg = format!("remote and local are same: {:?}", path);
                    println!("{}", msg.green());
                }
            } else {
                let msg = format!("{:?} dont's has remote", path);
                println!("{}", msg.yellow());
            }
        }
        NotGit(path) => {
            let msg = format!("{:?}", path);
            println!("{}", msg.red());
        }
    })
}
