use std::io::{Result as IoResult, Read};
use std::os::linux::fs::MetadataExt;
use std::path::{PathBuf, Path};
use std::fs::{Metadata, File};

mod tar;
mod untar;

#[derive(Debug, Clone)]
pub(crate) struct ArchiveFile {
    file_name: PathBuf,
    metadata: Metadata,
    content: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Archive {
    files: Vec<ArchiveFile>,
}

impl ArchiveFile {
    fn new<T: AsRef<Path>>(file_name: &T) -> IoResult<Self> {
        let mut file = File::open(file_name)?;
        let metadata = file.metadata()?;
        let file_size = metadata.st_size() as usize;
        let mut content = vec![0u8; file_size];

        if file.read_to_end(&mut content)? != file_size {
            eprintln!(r"\`st_size\` and \`file_size\` are not equal");
            std::process::exit(1);
        }
        

        Ok(
            Self {
                file_name: file_name.as_ref().to_path_buf(),
                metadata,
                content,
            }
        )
    }
}