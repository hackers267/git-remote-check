pub mod dir;
pub mod git;
use std::path::Path;

#[derive(Debug)]
pub enum Git {
    Git(Box<Path>),
    NotGit(Box<Path>),
}
