mod lib;

use crate::lib::{
    dir::is_remote_local_syuc,
    dir::walk_dirs,
    git::has_remote,
    Git::{Local, No, Remote, RemoteSync},
};
use colored::Colorize;
use std::path::Path;

fn main() {
    let mut args = std::env::args();
    let root = args.nth(1).unwrap_or(".".to_string());
    let dirs = walk_dirs(Path::new(&root));
    dirs.iter().for_each(|dir| match dir {
        Local(path) => {
            let msg = format!("{:?} dont's has remote", path);
            println!("{}", msg.yellow());
        }
        No(path) => {
            let msg = format!("{:?}", path);
            println!("{}", msg.red());
        }
        Remote(path) => {
            let message = format!("remote and local are different: {:?}", path);
            println!("{}", message.magenta());
        }
        RemoteSync(path) => {
            let msg = format!("remote and local are same: {:?}", path);
            println!("{}", msg.green());
        }
    })
}
