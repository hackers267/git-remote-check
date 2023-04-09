use colored::Colorize;
use grc::dir::walk_dirs;
use grc::Git;
use grc::Git::{Local, No, Remote, RemoteSync};
use std::path::Path;

fn main() {
    let mut args = std::env::args();
    let root = args.nth(1).unwrap_or(".".to_string());
    let dirs = walk_dirs(Path::new(&root));
    print_remote_sync(&dirs);
    print_remote(&dirs);
    print_local(&dirs);
    print_no(&dirs);
}

fn print_remote_sync(dirs: &[Git]) {
    dirs.iter()
        .filter_map(|dir| match dir {
            RemoteSync(path) => Some(format!("本地和远程已同步:{:?}", path)),
            _ => None,
        })
        .map(|str| str.green())
        .for_each(|str| println!("{}", str));
}

fn print_remote(dirs: &[Git]) {
    dirs.iter()
        .filter_map(|dir| match dir {
            Remote(path) => Some(format!("本地和远程未同步：{:?}", path)),
            _ => None,
        })
        .map(|str| str.red())
        .for_each(|str| println!("{}", str));
}

fn print_no(dirs: &[Git]) {
    dirs.iter()
        .filter_map(|dir| match dir {
            No(path) => Some(format!("未使用Git仓库：{:?}", path)),
            _ => None,
        })
        .map(|str| str.red())
        .for_each(|str| println!("{}", str));
}

fn print_local(dirs: &[Git]) {
    dirs.iter()
        .filter_map(|dir| match dir {
            Local(path) => Some(format!("未设置远程仓库：{:?}", path)),
            _ => None,
        })
        .map(|str| str.magenta())
        .for_each(|str| println!("{}", str));
}
