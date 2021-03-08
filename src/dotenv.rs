use crate::utils;
use std::collections::HashMap;
use std::io;

#[derive(Clone)]
pub struct EnvVar {
    pub key: String,
    pub value: String,
}
pub type EnvVarsMap = HashMap<String, String>;

#[derive(Debug)]
pub enum ParseError {
    IOError(io::Error),
    Error(String)
}

impl From<io::Error> for ParseError {
    fn from(e: io::Error) -> ParseError {
        return ParseError::IOError(e);
    }
}

impl From<String> for ParseError {
    fn from(e: String) -> ParseError {
        return ParseError::Error(e);
    }
}


pub fn parse(path: &std::path::PathBuf) -> Result<HashMap<String, String>, crate::CustomError> {
    let mut env_vars : EnvVarsMap = HashMap::new();

    let lines = utils::read_lines(path)
        .map_err(|err| crate::CustomError(format!("fail reading lines of `{}`:`{}`", path.to_str().unwrap(), err)))?;

    for ( i, line ) in lines.enumerate() {
        let line = line
            .map_err(|err| crate::CustomError(format!("fail reading line `{}` of `{}`: `{}`", i, path.to_str().unwrap(), err)))?;

        let equal_sign = line.clone().find('=');
        if equal_sign == None {
           return Err(crate::CustomError(format!("invalid line `{}` of `{}`, missing `=`", i, path.to_str().unwrap())));
        }

        let kv: Vec<&str> = line.splitn(2, '=').collect();

        let (key, value) = match &kv[..] {
            &[first, second, ..] => Ok((first, second)),
            _ => Err(crate::CustomError(format!("failed reading line `{}` of `{}`", i, path.to_str().unwrap()))),
        }?;

        env_vars.insert(String::from(key), String::from(value));
    }

    Ok(env_vars)
}
