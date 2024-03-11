use std::path::PathBuf;

use blake3::Hash;

pub struct SourceFile {
    pub id: Hash,
    pub name: String,
    pub path: PathBuf,
}

pub struct SourcePackage {
    pub id: Hash,
    pub name: String,
    pub bpath: PathBuf,
}

pub struct SourceTree {}
