use std::io::Result as IoResult;
use std::path::Path;
use crate::{Archive, ArchiveFile};

impl Archive {
    fn create(files: &[&Path]) -> IoResult<Self> {
        let mut files_vec = Vec::with_capacity(files.len());
        for file in files {
            files_vec.push(ArchiveFile::new(file)?);
        }

        Ok(
            Self {
                files: files_vec
            }
        )
    }
}