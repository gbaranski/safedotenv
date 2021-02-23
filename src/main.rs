use structopt::StructOpt;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use std::fs::File;


#[derive(Debug, StructOpt)]
struct Args {
    pub path: String,
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn scan_line(line: String, env: &str) -> Option<usize> {
    if line.len() < env.len() {
        return None
    };
    let mut i: usize = 0;
    let mut j: usize = 0;

    while i < line.len() && j < env.len() {
        if line.chars().nth(i) == env.chars().nth(j) {
            i += 1;
            j += 1;
            if j == env.len() {
                return Some(i - env.len());
            }
        } else {
            i = i - j + 1;
            j = 0;
        }
    }
    return None
}

fn scan_file(path: String, envs: Vec<&str>) {
    let lines = read_lines(path).unwrap();
    for (line_count, line) in lines.enumerate() {
        let line = line.unwrap();
        for env in &envs {
            let leak = scan_line(line.clone(), env);
            match leak {
                Some(char_count) => println!("Possible leak at {}:{}", line_count, char_count),
                None             => println!("No leak at line {}", line_count)
            }
        }
    }
}

fn scan_dir(path: String, envs: Vec<&str>) -> std::io::Result<()> {
    let paths = fs::read_dir(path).unwrap();
    for dir_entry in paths {
        let dir_entry = dir_entry.unwrap();
        let path_str = dir_entry.path().to_str().unwrap().to_string();
        let md = dir_entry.metadata().unwrap();
        if md.is_dir() {
            println!("{} is directory, searching recursively", path_str);
            return scan_dir(path_str, envs);
        }
        println!("{} is file, searching for leak", path_str);
        scan_file(path_str, envs.clone());
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let args = Args::from_args();
    let envs = ["hello"];
    scan_dir(args.path, envs.to_vec())
}
