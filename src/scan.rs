use colored::*;
use crate::utils::read_lines;
use crate::dotenv::EnvVar;
use std::path::PathBuf;
use std::collections::HashMap;

pub struct FoundEnvVar {
    pub env: EnvVar,
    pub line_n: usize,
    pub char_n: usize,

    pub path: PathBuf,
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
             found.path.to_str().unwrap().bold(), 
             (found.line_n + 1).to_string().bold(), 
             (found.char_n + 1).to_string().bold(), 
             "found".red().bold(),
             found.env.key.bright_red().bold(),
             );

    println!("{} | {}", found.line_n + 1, censored_line);
}

fn scan_line(line: String, value: &String) -> Option<usize> {
    if line.len() < value.len() {
        return None
    };
    let mut i: usize = 0;
    let mut j: usize = 0;

    while i < line.len() && j < value.len() {
        if line.chars().nth(i) == value.chars().nth(j) {
            i += 1;
            j += 1;
            if j == value.len() {
                return Some(i - value.len());
            }
        } else {
            i = i - j + 1;
            j = 0;
        }
    }
    return None
}


pub fn scan_file<'a>(
    path: PathBuf, 
    envs: HashMap<String, String>, 
    ) -> Result<Vec<FoundEnvVar>, crate::CustomError> {
    let mut found_envs: Vec<FoundEnvVar> = vec![];

    let lines = read_lines(path.clone())
        .map_err(|err| crate::CustomError(format!("fail reading lines of `{}`: `{}`", path.to_str().unwrap(), err)))?;

    for (line_n, line) in lines.enumerate() {
        let line = line
            .map_err(|err| crate::CustomError(
                    format!(
                        "fail reading line `{}` of `{}`: {}", line_n + 1, path.to_str().unwrap(), err)
                    )
                )?;

        for env in &envs {
            let (key, value) = env;
            let scanned_line = scan_line(line.clone(), value);
            match scanned_line {
                Some(char_n) => {
                    let found_env = FoundEnvVar {
                        line: line.clone(),
                        line_n,
                        char_n,
                        env: EnvVar{
                            key: key.clone(),
                            value: value.clone(),
                        },
                        path: path.clone(),
                    };
                    found_envs.push(found_env);
                }
                None => {}
            }
        }
    }

    Ok(found_envs)
}

// pub fn scan_dir(path: &PathBuf, envs: EnvVarsMap) -> Result<(), crate::CustomError> {
//     let paths = fs::read_dir(path)
//         .map_err(|err| crate::CustomError(format!("Error reading directory `{}`: {}", path.to_str().unwrap(), err)))?;

//     for dir_entry in paths {
//         let dir_entry = dir_entry
//             .map_err(|err| crate::CustomError(format!("failed reading directory entry: {}", err)))?;

//         let path = dir_entry.path();
//         let md = path.metadata()
//             .map_err(|err| crate::CustomError(format!("failed scanning metadata of file `{}`: {}",path.to_str().unwrap() ,err)))?;
//         if md.is_dir() {
//             return scan_dir(&path, envs);
//         }
//         println!("scanning: {}", path.to_str().unwrap().bold());
//         scan_file(&path, envs.clone())?;
//     }
//     Ok(())
// }

