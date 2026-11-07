use crate::error::Result;
use std::{
    fs::File,
    io::{self, Read, Write},
    path::Path,
};

pub fn input(path: Option<&Path>) -> Result<Box<dyn Read>> {
    match path.filter(|p| *p != Path::new("-")) {
        Some(path) => Ok(Box::new(io::BufReader::new(File::open(path)?))),
        None => Ok(Box::new(io::stdin())),
    }
}
pub fn output(path: Option<&Path>) -> Result<Box<dyn Write>> {
    match path {
        Some(path) => Ok(Box::new(io::BufWriter::new(File::create(path)?))),
        None => Ok(Box::new(io::stdout())),
    }
}
pub fn read(mut reader: impl Read) -> Result<String> {
    let mut bytes = Vec::new();
    reader.read_to_end(&mut bytes)?;
    Ok(String::from_utf8(bytes)?)
}
