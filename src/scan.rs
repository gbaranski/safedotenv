use std::path::PathBuf;
use std::collections::HashMap;
use std::fmt;
use colored::Colorize;
use crate::utils::{read_lines, Censorable};
use crate::dotenv::EnvVar;

pub struct FoundEnvVar {
    pub env: EnvVar,
    pub line_n: usize,
    pub char_n: usize,

    pub path: PathBuf,
    pub line: String,
}


impl std::fmt::Display for FoundEnvVar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let censored = self.line.censor(
            self.char_n,
            self.char_n + self.env.value.len()
            );

        writeln!(f, "{}:{}:{}: {} {}", 
                 self.path.to_str().unwrap().bold(), 
                 (self.line_n + 1).to_string().bold(), 
                 (self.char_n + 1).to_string().bold(), 
                 "found".red().bold(),
                 self.env.key.bright_red().bold(),
                 )?;
        write!(f, "{} | {}", self.line_n + 1, censored)?;

        Ok(())
    }
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
    log::debug!("scanning file {}", path.to_str().unwrap());
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

