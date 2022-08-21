use std::io::{prelude::*, Result as IoResult};
use std::fs::File;
use std::path::Path;
use crate::Archive;

impl Archive {
    pub fn extract<T: AsRef<Path>>(file: &T) -> IoResult<()> {
        const USIZE_SIZE: usize = std::mem::size_of::<usize>();
        let mut file = File::open(file)?;
        let mut buf = vec![];
        file.read_to_end(&mut buf)?;

        let mut start: usize = 0;

        // Kinda bad to read... ik
        while start < buf.len() {
            let file_name_len = usize::from_ne_bytes(buf[start..start+USIZE_SIZE].try_into().unwrap());
    
            let file_name_offset = start+USIZE_SIZE+file_name_len;
    
            let file_name = std::str::from_utf8(&buf[start+USIZE_SIZE..file_name_offset]).unwrap();
    
            let file_size = usize::from_ne_bytes(buf[file_name_offset..file_name_offset+USIZE_SIZE].try_into().unwrap());    

            let file_contents_offset = file_name_offset+USIZE_SIZE+file_size;

            let file_contents = &buf[file_name_offset+USIZE_SIZE..file_contents_offset];
    
            File::create(file_name)?.write_all(file_contents)?;
            start = file_contents_offset;
        }


        Ok(())
    }
}