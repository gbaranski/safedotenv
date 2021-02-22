use structopt::StructOpt;
use std::fs;

#[derive(Debug, StructOpt)]
struct Args {
    pub path: String,
}

fn scan_file(path: String, envs: Vec<String>) {
    let file = fs::read(path).unwrap();
    for env in envs {
        for i in 0..file.len() {
            if i + env.len() > file.len() {
                break;
            }

            let mut j = 0;
            let is_leak = file[i..i+env.len()].iter().all(|&b| {
                let corresponding_env_byte = env.as_bytes()[j];
                j += 1;
                return corresponding_env_byte == b;
            });
            if is_leak {
                println!("Found possible leak at character {}", i);
            }
        }
    }

}

fn scan_dir(path: String, envs: Vec<String>) -> std::io::Result<()> {
    let paths = fs::read_dir(path).unwrap();
    for dir_entry in paths {
        let dir_entry = dir_entry.unwrap();
        let path_str = dir_entry.path().to_str().unwrap().to_string();
        let md = dir_entry.metadata().unwrap();
        if md.is_dir() {
            println!("{} is directory, searching recursively", path_str);
            return scan_dir(path_str, envs);
        }
        println!("{} is file, searching for leak", path_str);
        scan_file(path_str, envs.clone());
    }
    Ok(())
}

fn main() {
    let args = Args::from_args();
    let envs = ["hello".to_string()];
    panic!(scan_dir(args.path, envs.to_vec()));
}
