use std::io::{Error, ErrorKind};
use structopt::StructOpt;
use human_panic::setup_panic;

pub mod utils;
pub mod scan;
pub mod dotenv;
pub mod cli;


fn main() -> std::io::Result<()> {
    setup_panic!();

    let options: cli::Options = cli::Options::from_args();
    cli::Options::from_args();

    for path in &options.paths {
        let path_type = utils::get_path_type(path)?;
        
        let dotenv_path = match path_type {
            utils::PathType::Directory => 
                Ok(options.env_file.clone().unwrap_or(format!("{}/.env", path.to_str().unwrap()))),
            utils::PathType::File => 
                if options.env_file == None {
                    Err(
                        Error::new(
                            ErrorKind::InvalidInput, 
                            "env_file param is missing, it is required if input is directory"))
                } else {
                    Ok(options.env_file.clone().unwrap())
                }
        }?;

        let env_vars = dotenv::parse(&dotenv_path);
        assert!(env_vars.is_ok(), "failed reading env_vars from '{}' {}", dotenv_path, env_vars.unwrap_err());
        let env_vars = env_vars.unwrap();

        match path_type {
            utils::PathType::Directory => scan::scan_dir(path, env_vars),
            utils::PathType::File => scan::scan_file(path, env_vars),
        }?;

    }

    Ok(())
}
