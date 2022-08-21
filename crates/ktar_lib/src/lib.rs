use std::io::{Result as IoResult, Read};
use std::os::linux::fs::MetadataExt;
use std::path::Path;
use std::fs::File;

mod tar;
mod untar;

#[derive(Debug, Clone)]
pub(crate) struct ArchiveFile {
    file_name: String,
    file_size: usize,
    contents: Vec<u8>,
}

#[derive(Debug, Clone, Copy)]
pub struct Archive;

impl ArchiveFile {
    fn new<T: AsRef<Path>>(file_name: &T) -> IoResult<Self> {
        let mut file = File::open(file_name)?;
        let metadata = file.metadata()?;
        let file_size = metadata.st_size() as usize;
        let mut contents = Vec::with_capacity(file_size);
        if file.read_to_end(&mut contents)? != file_size {
            eprintln!(r"\`st_size\` and \`file_size\` are not equal");
            std::process::exit(1);
        }
        
        Ok(
            Self {
                file_name: file_name
                    .as_ref()
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
                file_size,
                contents,
            }
        )
    }
}