use structopt::StructOpt;
use std::collections::HashMap;

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



fn main() -> Result<(), CustomError> {
    let options: cli::Options = cli::Options::from_args();

    options
        .init_logging()
        .map_err(|err| CustomError(
                format!(
                    "fail initializing logging: `{}`", err)
                ))?;

    let dotenv_path = dotenv::get_dotenv_path(&options)?;
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
            .filter(|entry| 
                    entry
                    .file_type()
                    .ok_or(
                        CustomError(
                            format!("file `{}` has missing file type", entry.path().to_str().unwrap()))
                        )
                    .expect("missing filetype")
                    .is_file()
                   );

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
        println!("{}", found_env);
    }


    Ok(())
}
