use structopt::StructOpt;
use colored::*;

#[derive(Debug)]
pub struct CustomError(String);

pub mod utils;
pub mod scan;
pub mod dotenv;
pub mod cli;

enum Work {
    File(std::path::PathBuf),
    Quit
}

use std::collections::HashMap;

struct Worker {
    chan: deque::Stealer<Work>,
    env_vars: HashMap<String, String>
}

impl<'a> Worker {
    pub fn run(self) -> Vec<scan::FoundEnvVar> {
        let mut v: Vec<scan::FoundEnvVar> = vec![];
        loop {
            match self.chan.steal() {
                deque::Stolen::Empty | deque::Stolen::Abort => continue,
                deque::Data(Work::Quit) => break,
                deque::Data(Work::File(path)) => {
                    let found = scan::scan_file(path, self.env_vars.clone()).unwrap();
                    v.extend(found);
                }
            };
        }
        return v;
    }
}


fn get_dotenv_path(targets: &Vec<std::path::PathBuf>) -> Option<std::path::PathBuf> {
    for target in targets {
        if target.is_dir() {
            let mut possible_dotenv_path = target.clone();
            possible_dotenv_path.push(".env");
            if possible_dotenv_path.exists() {
                return Some(possible_dotenv_path);
            }
        }
    }

    None
}

fn main() -> Result<(), CustomError> {
    let options: cli::Options = cli::Options::from_args();

    let dotenv_path = match options.env_file {
        Some(path) => Ok(path),
        None => {
            match get_dotenv_path(&options.targets) {
                Some(path) => {
                    println!("Found .env file at {}, optionally you can specify it using --env-file", path.parent().unwrap().to_str().unwrap());
                    Ok(path)
                },
                None => Err(CustomError("unable to find any .env file in targets, specify one using --env-file flag".to_string()))
            }
        }
    }?;
    let env_vars = dotenv::parse(&dotenv_path)?;


    let threads = num_cpus::get();
    let mut workers = vec![];
    let (workq, stealer) = deque::new();
    for _ in 0..threads {
        let worker = Worker { 
            chan: stealer.clone(),
            env_vars: env_vars.clone(),
        };
        workers.push(std::thread::spawn(|| worker.run()));
    }
    
    for target in options.targets {
        let walker = ignore::WalkBuilder::new(target).build();
        let dir_entries = walker
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().expect("missing filetype").is_file());
        for dir_entry in dir_entries {
            workq.push(Work::File(dir_entry.into_path()));
        }
    }

    for _ in 0..workers.len() {
        workq.push(Work::Quit);
    }
    let mut found_envs: Vec<scan::FoundEnvVar> = Vec::new();
    for worker in workers {
        found_envs.extend(worker.join().unwrap());
    }

    for found_env in found_envs {
        let censored_line: String = found_env.line
            .chars()
            .enumerate()
            .map(|(i, c)| 
                 if i >= found_env.char_n && i < found_env.char_n + found_env.env.value.len() {'*'} else {c})
            .collect();

        println!("{}:{}:{}: {} {}", 
                 found_env.path.to_str().unwrap().bold(), 
                 (found_env.line_n + 1).to_string().bold(), 
                 (found_env.char_n + 1).to_string().bold(), 
                 "found".red().bold(),
                 found_env.env.key.bright_red().bold(),
                 );

        println!("{} | {}", found_env.line_n + 1, censored_line);
    }


    Ok(())
}
