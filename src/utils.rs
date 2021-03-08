use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub enum PathType {
    File,
    Directory,
}


pub fn get_path_type(path: &std::path::PathBuf) -> Result<PathType, crate::CustomError> {
    if path.is_dir() {
        Ok(PathType::Directory)
    } else if path.is_file() {
        Ok(PathType::File)
    } else {
        Err(crate::CustomError(format!("path `{}` is neither directory nor file", path.to_str().unwrap())))
    }
}

pub trait Censorable {
    fn censor(&self, start: usize, end: usize) -> String;
}

impl Censorable for String {
    fn censor(&self, start: usize, end: usize) -> String {
        self.chars()
            .enumerate()
            .map(|(i, c)| 
                 if i >= start && i < end {'*'} else {c})
            .collect()
    }
}
