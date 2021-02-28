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


pub fn get_path_type(path: &String) -> Result<PathType, std::io::Error> {
    let md = std::fs::metadata(path)?;
    if md.is_dir() {
        Ok(PathType::Directory)
    } else if md.is_file() {
        Ok(PathType::File)
    } else {
        Err(
            std::io::Error::new(
                std::io::ErrorKind::InvalidData, "path is neither directory nor file"
                )
           )
    }
}

