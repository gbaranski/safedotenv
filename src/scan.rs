use std::path::PathBuf;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;
use std::io::prelude::*;
use std::fmt;
use colored::Colorize;
use aho_corasick::AhoCorasick;
use crate::utils::{Censorable, Line};
use crate::dotenv::EnvVar;

pub struct FoundEnvVar {
    pub env: EnvVar,
    pub path: PathBuf,
    pub line: Line,
}


impl std::fmt::Display for FoundEnvVar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let censored = self.line.content.censor(
            self.line.column,
            self.line.column + self.env.value.len()
            );

        writeln!(f, "{}:{}:{}: {} {}", 
                 self.path.to_str().unwrap().bold(), 
                 (self.line.row + 1).to_string().bold(), 
                 (self.line.column + 1).to_string().bold(), 
                 "found".red().bold(),
                 self.env.key.bright_red().bold(),
                 )?;
        write!(f, "{} | {}", self.line.row + 1, censored)?;

        Ok(())
    }
}

pub fn scan_file<'a>(
    path: PathBuf, 
    envs: HashMap<String, String>, 
    ) {

    if path.is_dir() {
        return;
    }

    let file = File::open(&path).unwrap();

    let values = envs
        .iter()
        .map(|e| e.1);

    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    match buf_reader.read_to_string(&mut contents) {
        Ok(_size) => (),
        Err(_err) => return
    };

    let ac = AhoCorasick::new(values);

    for mat in ac.find_iter(&contents) {
        let env_tuple = envs
            .iter()
            .nth(mat.pattern())
            .unwrap();

        let line = crate::utils::find_line(&contents, mat.start(), mat.end());
        let found_env = FoundEnvVar {
            env: EnvVar {
                key: env_tuple.0.clone(),
                value: env_tuple.1.clone(),
            },
            line,
            path: path.clone(),
        };

        log::warn!("{}", found_env);
    }
}

