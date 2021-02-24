use structopt::StructOpt;
use std::fs;

pub mod utils;
pub mod scan;
pub mod dotenv;

#[derive(Debug, StructOpt)]
struct Args {
    pub path: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::from_args();
    let md = fs::metadata(args.path.clone())?;

    if md.is_dir() {
        let dotenv_path = format!("{}/.env", args.path);
        let env_vars = dotenv::parse(dotenv_path)?;

        scan::scan_dir(args.path, env_vars)?;
    } else {
        // scan::scan_file(args.path, envs.to_vec())
    }
    Ok(())
}
