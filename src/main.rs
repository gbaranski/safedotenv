use structopt::StructOpt;
use std::collections::HashMap;
use std::time::Instant;

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
                    let found = scan::scan_file(path, self.env_vars.clone());
                    match found {
                        Ok(found) => v.extend(found),
                        Err(err) => log::debug!("{:?}", err),

                    }
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
    log::debug!("will use dotenv file at `{}`", dotenv_path.to_str().unwrap());

    let env_vars = dotenv::parse(&dotenv_path)?;
    if log::max_level() >= log::LevelFilter::Debug {
        for (i, env) in env_vars.iter().enumerate() {
            let (key, value) = env;
            log::debug!("{} env: {}={}", i, key, value);
        }
    }

    let threads = num_cpus::get();
    let mut workers = vec![];
    let (workq, stealer) = deque::new();
    for i in 0..threads {
        let worker = Worker { 
            chan: stealer.clone(),
            env_vars: env_vars.clone(),
        };
        workers.push(std::thread::spawn(|| worker.run()));

        log::debug!("spawning worker {}", i);
    }


    let pre_scan = Instant::now();

    let all_entries = options.targets
        .into_iter()
        .map(|target|
             ignore::WalkBuilder::new(target)
             .build()
             .filter_map(|entry| entry.ok())
             .filter(|entry| entry
                     .file_type()
                     .unwrap()
                     .is_file())
            ).flatten().collect::<Vec<ignore::DirEntry>>();

    for dir_entry in &all_entries {
        workq.push(Work::File(dir_entry.clone().into_path()));
    }

    for _ in 0..workers.len() {
        workq.push(Work::Quit);
    }
    let mut found_envs: Vec<scan::FoundEnvVar> = Vec::new();
    for worker in workers {
        found_envs.extend(worker.join().unwrap());
    }
    let elapsed = pre_scan.elapsed();

    for found_env in found_envs {
        log::warn!("{}", found_env)
    }

    log::info!("Scanned {} files in {:?}", all_entries.len(),elapsed);


    Ok(())
}
