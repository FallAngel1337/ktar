use std::io::{prelude::*, Result as IoResult};
use std::fs::File;
use std::path::PathBuf;
use crate::{Archive, ArchiveFile};

impl Archive {
    fn tarball(files: &[PathBuf]) -> IoResult<Vec<u8>> {
        let mut tarball = vec![];
        for file in files {
            let archive = ArchiveFile::new(file)?;
            let (
                file_name,
                file_name_len,
                file_size,
                file_contemts
            ) = (
                archive.file_name.as_bytes(),
                &archive.file_name.len().to_ne_bytes(),
                &archive.file_size.to_ne_bytes(),
                &archive.contents);
            
            println!("{:?}, {:?}, {:?}", std::str::from_utf8(file_name), usize::from_ne_bytes(*file_name_len), usize::from_ne_bytes(*file_size));

            tarball.extend_from_slice(&[file_name_len, file_name, file_size, file_contemts].concat())
        }

        Ok(tarball)
    }

    pub fn create(files: &[PathBuf], output: PathBuf) -> IoResult<()> {
        let tarball = Self::tarball(files)?;
        let mut file = File::create(output)?;
        file.write_all(&tarball)?;
        Ok(())
    }
}