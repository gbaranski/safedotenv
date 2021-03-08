use structopt::StructOpt;

/// Scans your code for any secret leaks and alerts you about it
#[derive(StructOpt, Debug)]
#[structopt(name="safedotenv", author="gbaranski <root@gbaranski.com>", version="1.0")]
pub struct Options {
    #[structopt(help = "Set input file/directory to scan", parse(from_os_str))]
    pub targets: Vec<std::path::PathBuf>,


    #[structopt(short = "f", long, help="Set dotenv file to read from(by default <INPUT>/.env)")]
    pub env_file: Option<std::path::PathBuf>,


    #[structopt(long, help="Set files/directories to ignore")]
    pub ignored_files: Vec<String>,

    #[structopt(long, help="Set enviroment variables to ignore")]
    pub ignored_envs: Vec<String>,
}

