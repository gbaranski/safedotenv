use structopt::StructOpt;
use std::fs;

pub mod utils;
pub mod scan;

#[derive(Debug, StructOpt)]
struct Args {
    pub path: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::from_args();
    let envs = [
        scan::EnvVar{
            key: "SECRET_TOKEN".to_string(),
            value: "hello".to_string(),
        }
    ];
    let md = fs::metadata(args.path.clone())?;
    if md.is_dir() {
        scan::scan_dir(args.path, envs.to_vec())
    } else {
        scan::scan_file(args.path, envs.to_vec())
    }
}
