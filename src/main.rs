use structopt::StructOpt;

#[derive(Debug)]
pub struct CustomError(String);

pub mod utils;
pub mod scan;
pub mod dotenv;
pub mod cli;


fn main() -> Result<(), CustomError> {
    let options: cli::Options = cli::Options::from_args();
    cli::Options::from_args();

    for path in &options.paths {
        let path_type = utils::get_path_type(path)?;
        
        let dotenv_path = match path_type {
            utils::PathType::Directory => 
                Ok(options.env_file.clone().unwrap_or(format!("{}/.env", path.to_str().unwrap()))),
            utils::PathType::File => 
                if options.env_file == None {
                    Err(CustomError(String::from("unknown env file path, specify it by --env-file argument, or target a directory with .env file")))
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
