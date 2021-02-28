use crate::utils;
use std::collections::HashMap;
use std::io;

#[derive(Clone, Copy)]
pub struct EnvVar<'a > {
    pub key: &'a String,
    pub value: &'a String,
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


pub fn parse(path: &str) -> io::Result<EnvVarsMap> {
    let mut env_vars : EnvVarsMap = HashMap::new();

    let lines = utils::read_lines(path)?;
    for ( i, line ) in lines.enumerate() {
        let line = line?;
        let equal_sign = line.clone().find('=');
        if equal_sign == None {
           let err = io::Error::new(io::ErrorKind::InvalidInput, format!("line {} is invalid dotenv variable, missing '='", i));
           return Err(err)
        }

        let kv: Vec<&str> = line.splitn(2, '=').collect();

        let (key, value) = match &kv[..] {
            &[first, second, ..] => Ok((first, second)),
            _ => Err(io::Error::new(io::ErrorKind::InvalidInput, format!("invalid env var at line {}", i))),
        }?;

        env_vars.insert(String::from(key), String::from(value));
    }

    Ok(env_vars)
}
