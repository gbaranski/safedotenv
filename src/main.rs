use structopt::StructOpt;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use std::fs::File;
use colored::*;


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

struct LeakedEnv<'a> {
    pub line_n: usize,
    pub char_n: usize,

    pub path: String,
    pub line: String,
    pub env: &'a str,
}


fn leak_alert(env: &LeakedEnv) {
    let censored_line: String = env.line
        .chars()
        .enumerate()
        .map(|(i, c)| 
             if i >= env.char_n && i < env.char_n + env.env.len() {'*'} else {c})
        .collect();

    println!("{}:{}:{}: {}", 
             env.path.bold(), 
             (env.line_n + 1).to_string().bold(), 
             (env.char_n + 1).to_string().bold(), 
             "Possible leak".red().bold());

    println!("{} | {}", env.line_n + 1, censored_line);
}

fn scan_file(path: String, envs: Vec<&str>) -> std::io::Result<()> {
    let lines = read_lines(path.clone())?;
    for (line_n, line) in lines.enumerate() {
        let line = line?;
        for env in &envs {
            let scanned_line = scan_line(line.clone(), env);
            match scanned_line {
                Some(char_n) => {
                    let leaked_env = LeakedEnv {
                        line: line.clone(),
                        line_n,
                        char_n,
                        env,
                        path: path.clone(),
                    };
                    leak_alert(&leaked_env);
                }
                None => {}            
            }
        }
    }
    Ok(())
}

fn scan_dir(path: String, envs: Vec<&str>) -> std::io::Result<()> {
    let paths = fs::read_dir(path)?;
    for dir_entry in paths {
        let dir_entry = dir_entry?;
        let path_str = dir_entry.path().to_str().unwrap().to_string();
        let md = dir_entry.metadata()?;
        if md.is_dir() {
            return scan_dir(path_str, envs);
        }
        println!("scanning: {}", path_str.bold());
        scan_file(path_str, envs.clone())?;
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let args = Args::from_args();
    let envs = ["hello"];
    scan_dir(args.path, envs.to_vec())
}
