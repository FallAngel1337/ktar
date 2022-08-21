use std::io::Result as IoResult;
use std::path::PathBuf;
use crate::{Archive, ArchiveFile};

impl Archive {
    pub fn create(files: &[PathBuf]) -> IoResult<Vec<u8>> {
        let mut _files = vec![];
        for file in files {
            _files.push(ArchiveFile::new(file)?);
        }
        let files = _files;

        Ok(vec![])
    }
}