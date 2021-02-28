use structopt::StructOpt;
use std::io::{Error, ErrorKind};

pub mod utils;
pub mod scan;
pub mod dotenv;


/// Scans your code for any secret leaks and alerts you about it
#[derive(StructOpt)]
#[structopt(name="safedotenv", author="gbaranski <root@gbaranski.com>", version="1.0")]
struct Options {
    #[structopt(help = "Set input file/directory to scan")]
    pub paths: Vec<String>,


    #[structopt(short = "f", long, help="Set dotenv file to read from(by default <INPUT>/.env)")]
    pub env_file: Option<String>,


    #[structopt(long, help="Set files/directories to ignore")]
    pub ignored_files: Vec<String>,

    #[structopt(long, help="Set enviroment variables to ignore")]
    pub ignored_envs: Vec<String>,
}



fn main() -> std::io::Result<()> {
    let options: Options = Options::from_args();

    for path in &options.paths {
        let path_type = utils::get_path_type(path)?;
        
        let dotenv_path = match path_type {
            utils::PathType::Directory => Ok(options.env_file.clone().unwrap_or(format!("{}/.env", path))),
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
