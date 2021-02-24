use std::fs;
use colored::*;
use crate::utils::read_lines;
use crate::dotenv::{EnvVar, EnvVarsMap};

struct FoundEnvVar<'a> {
    pub env: EnvVar<'a>,
    pub line_n: usize,
    pub char_n: usize,

    pub path: String,
    pub line: String,
}



fn alert_found_env(found: &FoundEnvVar) {
    let censored_line: String = found.line
        .chars()
        .enumerate()
        .map(|(i, c)| 
             if i >= found.char_n && i < found.char_n + found.env.value.len() {'*'} else {c})
        .collect();

    println!("{}:{}:{}: {} {}", 
             found.path.bold(), 
             (found.line_n + 1).to_string().bold(), 
             (found.char_n + 1).to_string().bold(), 
             "found".red().bold(),
             found.env.key.bright_red().bold(),
             );

    println!("{} | {}", found.line_n + 1, censored_line);
}

fn scan_line(line: String, env: EnvVar) -> Option<usize> {
    if line.len() < env.value.len() {
        return None
    };
    let mut i: usize = 0;
    let mut j: usize = 0;

    while i < line.len() && j < env.value.len() {
        if line.chars().nth(i) == env.value.chars().nth(j) {
            i += 1;
            j += 1;
            if j == env.value.len() {
                return Some(i - env.value.len());
            }
        } else {
            i = i - j + 1;
            j = 0;
        }
    }
    return None
}


pub fn scan_file(path: String, envs: EnvVarsMap) -> std::io::Result<()> {
    let lines = read_lines(path.clone())?;
    for (line_n, line) in lines.enumerate() {
        let line = line?;
        for env in &envs {
            let env = EnvVar{
                key: env.0,
                value: env.1,
            };
            let scanned_line = scan_line(line.clone(), env);
            match scanned_line {
                Some(char_n) => {
                    let found_env = FoundEnvVar {
                        line: line.clone(),
                        line_n,
                        char_n,
                        env,
                        path: path.clone(),
                    };
                    alert_found_env(&found_env);
                }
                None => {}            
            }
        }
    }
    Ok(())
}

pub fn scan_dir(path: String, envs: EnvVarsMap) -> std::io::Result<()> {
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

