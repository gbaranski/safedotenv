use std::collections::HashMap;
use std::path::PathBuf;

use crate::cli;
use crate::CustomError;
use crate::utils::read_lines;

#[derive(Clone)]
pub struct EnvVar {
    pub key: String,
    pub value: String,
}
pub type EnvVarsMap = HashMap<String, String>;

pub fn parse(path: &PathBuf) -> Result<HashMap<String, String>, CustomError> {
    let mut env_vars : EnvVarsMap = HashMap::new();

    let lines = read_lines(path)
        .map_err(|err| CustomError(format!("fail reading lines of `{}`:`{}`", path.to_str().unwrap(), err)))?;

    for ( i, line ) in lines.enumerate() {
        let line = line
            .map_err(|err| crate::CustomError(format!("fail reading line `{}` of `{}`: `{}`", i, path.to_str().unwrap(), err)))?;

        if line.len() < 1 {
            continue;
        }

        if line.chars().nth(0) == Some('#') {
            continue;
        }

        let equal_sign = line.clone().find('=');
        if equal_sign == None {
            return Err(CustomError(format!("invalid line `{}` of `{}`, missing `=`", i, path.to_str().unwrap())));
        }

        let kv: Vec<&str> = line.splitn(2, '=').collect();

        let (key, value) = match &kv[..] {
            &[first, second, ..] => Ok((first, second)),
            _ => Err(CustomError(format!("failed reading line `{}` of `{}`", i, path.to_str().unwrap()))),
        }?;

        env_vars.insert(String::from(key), String::from(value));
    }

    Ok(env_vars)
}

fn find_env_file_in_targets(targets: &Vec<std::path::PathBuf>) -> Option<std::path::PathBuf> {
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

pub fn get_dotenv_path(options: &cli::Options) -> Result<PathBuf, CustomError> {
    match options.env_file.clone() {
        Some(path) => Ok(path),
        None => {
            match find_env_file_in_targets(&options.targets) {
                Some(path) => {
                    log::info!(
                        "Found .env file at {}, optionally you can specify it using --env-file", 
                        path.parent().unwrap().to_str().unwrap()
                        );

                    Ok(path)
                },
                None => Err(
                    CustomError("unable to find any .env file in targets, specify one using --env-file flag".to_string())
                    )
            }
        }
    }
}
