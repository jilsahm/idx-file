use std::fs::File;
use std::io::{Read, BufReader, Seek, SeekFrom, Error};
use std::path::{Path, PathBuf};

/// IdxReader being able to read an idx vector file.
pub struct IdxReader {
    handle: PathBuf,
    datatype: u8,
    dimensions: u8,
}

impl IdxReader {
    pub fn new(path: &Path) -> Result<Self, Error> {
        let mut file = File::open(path)?;
        let mut byte = vec![0u8];
        file.seek(SeekFrom::Start(2))?; // Skipping the first two zero-bytes
        file.read(&mut byte)?;
        let datatype = byte[0];
        file.read(&mut byte)?;
        let dimensions = byte[0];

        Ok(IdxReader {
            handle: PathBuf::from(path),
            datatype,
            dimensions,
        })
    }
    pub fn iter(&mut self) -> Result<Iter<'_>, Error> {
        Ok(Iter {
            reader: self,
            buffer: BufReader::new(File::open(&self.handle)?),
        })
    }
}

pub struct Iter<'a> {
    reader: &'a IdxReader,
    buffer: BufReader<File>,
}
impl<'a> Iterator for Iter<'a> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        Some(0u8)
    }
}

#[cfg(test)]
mod tests {
    use super::{IdxReader, Path};
    use std::path::PathBuf;

    fn get_path_to_testfile() -> PathBuf {        
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("test/1.idx");
        path
    }

    #[test]
    fn test_idxreader_new_success() {
        assert!(IdxReader::new(&get_path_to_testfile()).is_ok());
    }

    #[test]
    fn test_idxreader_new_fail() {
        assert!(IdxReader::new(Path::new("void")).is_err());
    }

    #[test]
    fn test_idxreader_metadata() {
        let reader = IdxReader::new(&get_path_to_testfile()).unwrap();
        assert_eq!(8u8, reader.datatype);
        assert_eq!(1u8, reader.dimensions);
    }
}