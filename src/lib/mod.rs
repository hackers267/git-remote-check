pub mod dir;
pub mod git;
use std::path::Path;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Git {
    Local(Box<Path>),
    Remote(Box<Path>),
    RemoteSync(Box<Path>),
    No(Box<Path>),
}
