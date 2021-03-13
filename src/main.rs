use structopt::StructOpt;
use std::time::Instant;

#[derive(Debug)]
pub struct CustomError(String);

pub mod utils;
pub mod scan;
pub mod dotenv;
pub mod cli;

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

    let env_vars = dotenv::parse(&dotenv_path, &options.ignored_envs)?;
    if log::max_level() >= log::LevelFilter::Debug {
        for (i, env) in env_vars.iter().enumerate() {
            let (key, value) = env;
            log::debug!("{} env: {}={}", i, key, value);
        }
    }


    let start = Instant::now();

    options.targets
        .iter()
        .for_each(|target| {
            let walk_builder = ignore::WalkBuilder::new(target);
            walk_builder.build_parallel()
             .run(|| Box::new(|path| {
                 let dir_entry: ignore::DirEntry = path.unwrap().into();
                 scan::scan_file(dir_entry.into_path(), env_vars.clone());

                 ignore::WalkState::Continue
             }))
        }
            );

    log::info!("Scanned files in {:?}", start.elapsed());

    Ok(())
}
