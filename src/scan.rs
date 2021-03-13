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
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use yansi::Paint;

pub struct FoundEnvVar {
    pub env: EnvVar,
    pub path: PathBuf,
    pub line: Line,
}

impl FoundEnvVar {
    pub fn print(&self) -> std::io::Result<()> {
        let space = std::iter::repeat(' ')
            .take(self.line.row
                  .to_string()
                  .len())
            .collect::<String>();

        /*
           Logging inspired by rust compiler

           Leak of {key}
           --> {directory}:{row}:{column}
           |
           {row} | {content}
           |
           */

        let before_content = self.line.content.chars().take(self.line.column).collect::<String>();
        let censored_content = std::iter::repeat('*').take(self.env.value.len()).collect::<String>();
        let after_content = self.line.content.chars().skip(self.line.column + self.env.value.len()).collect::<String>();

        println!("{}\n{}\n{}\n{}\n{}", 
            format_args!("{} {}", 
                "leak of", 
                Paint::red(self.env.key.clone()).bold(), 
                ),
            format_args!("{} {}:{}:{}", 
                Paint::blue("-->"),
                self.path.display(),
                self.line.row + 1,
                self.line.column,
                ),
            format_args!("{} {}",
                space,
                Paint::blue("|")
                ),
            format_args!("{} {} {}{}{}",
                Paint::blue(self.line.row),
                Paint::blue("|"),
                before_content,
                Paint::red(censored_content),
                after_content,
                ),
            format_args!("{} {}",
                space,
                Paint::blue("|")
                )
        );

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
        found_env.print().unwrap();
    }
}

