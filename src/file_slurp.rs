extern crate std;
use std::io::prelude::*;
use std::path::Path;
use std::fs::File;

pub type Result<T> = std::result::Result<T, std::io::Error>;

pub fn write_file(filename: &str, str: &str) -> Result<()> {
    let path = Path::new(filename);
    let mut file = try!(File::create(&path));
    file.write_all(str.as_bytes())
}

pub fn read_file(filename: &str) -> Result<String> {
    let path = Path::new(filename);
    let mut file = try!(File::open(&path));
    let mut string = String::new();
    try!(file.read_to_string(&mut string));
    Ok(string)
}

//#[derive(Debug)]
//pub enum Error {
//    IoError(std::io::Error),
//}

#[cfg(test)]
mod tests {
    use file_slurp::*;
    #[test]
    fn test_write_file() {
        assert!(write_file("/tmp/test.txt","你好").is_ok());
        //println!("result: {:?}", write_file("/tmp/test.txt","hello"));
        assert_eq!(read_file("/tmp/test.txt").unwrap(), "你好".to_string());
        assert!(write_file("/root/test.txt","hello").is_err()); //TODO: should not use /root here
    }
}
