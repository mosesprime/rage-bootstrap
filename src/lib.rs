use std::path::PathBuf;

use source::SourceTree;

pub mod logger;
pub mod parse;
pub mod source;
pub mod token;

pub struct Build {
    src_path: PathBuf,
    src_tree: SourceTree,
}

impl Build {
    pub fn new(src_path: PathBuf) -> Self {
        Self {
            src_path,
            src_tree: todo!(),
        }
    }
}
