use structopt::StructOpt;
use colored::*;

#[derive(Debug)]
pub struct CustomError(String);

pub mod utils;
pub mod scan;
pub mod dotenv;
pub mod cli;

enum Work<'a> {
    File(&'a ignore::DirEntry),
    Quit
}

use std::collections::HashMap;

struct Worker<'a> {
    chan: deque::Stealer<Work<'a>>
}

impl<'a> Worker<'a> {
    pub fn run(self) -> Vec<scan::FoundEnvVar<'a>> {
        let mut v: Vec<scan::FoundEnvVar> = vec![];
        loop {
            match self.chan.steal() {
                deque::Stolen::Empty | deque::Stolen::Abort => continue,
                deque::Data(Work::Quit) => break,
                deque::Data(Work::File(dir_entry)) => {
                    let path = dir_entry.into_path();
                    v = scan::scan_file(&path, self.env_vars, v).unwrap();
                }
            };
        }

        v
    }
}


fn get_dotenv_path(targets: &Vec<std::path::PathBuf>) -> std::path::PathBuf {
    for target in targets {
        if target.is_dir() {
        }
    }

}

fn main() -> Result<(), CustomError> {
    let options: cli::Options = cli::Options::from_args();

    let dotenv_path = for target in option.targets {

    };

    // let dotenv_path = match utils::get_path_type(&options.path)? {
    //     utils::PathType::Directory => 
    //         Ok(options.env_file.clone().unwrap_or(format!("{}/.env", options.path.to_str().unwrap()))),
    //     utils::PathType::File => 
    //         if options.env_file == None {
    //             Err(CustomError(String::from("unknown env file path, specify it by --env-file argument, or target a directory with .env file")))
    //         } else {
    //             Ok(options.env_file.clone().unwrap())
    //         }
    // }?;
    // let env_vars = dotenv::parse(&dotenv_path);
    // assert!(env_vars.is_ok(), "failed reading env_vars from '{}' {}", dotenv_path, env_vars.unwrap_err());
    // let env_vars = env_vars.unwrap();


    let threads = num_cpus::get();
    let mut workers = vec![];
    let (workq, stealer) = deque::new();
    for _ in 0..threads {
        let worker = Worker { chan: stealer.clone() };
        workers.push(std::thread::spawn(|| worker.run()));
    }
    
    for target in options.targets {
        let walker = ignore::WalkBuilder::new(target).build();
        let paths = walker
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().expect("missing filetype").is_file());
        for path in paths {
            workq.push(Work::File(&path));
        }
    }

    for _ in 0..workers.len() {
        workq.push(Work::Quit);
    }
    let found_envs: Vec<scan::FoundEnvVar> = Vec::new();
    for worker in workers {
        let smth: Vec<scan::FoundEnvVar> = worker.join().unwrap().into();
        found_envs.extend(smth);
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
