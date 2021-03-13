use structopt::StructOpt;

/// Scans your code for any secret leaks and alerts you about it
#[derive(StructOpt, Debug)]
#[structopt(name="safedotenv", author="gbaranski <root@gbaranski.com>", version="1.0")]
pub struct Options {
    /// Set input file/directory to scan
    #[structopt(default_value=".", parse(from_os_str))]
    pub targets: Vec<std::path::PathBuf>,

    /// Enable debug mode(much more logging)
    #[structopt(short="d", long="--debug")]
    pub debug: bool,

    /// Set dotenv file to read from
    #[structopt(short = "f", long="--env-file")]
    pub env_file: Option<std::path::PathBuf>,

    /// Set enviroment variables to ignore
    #[structopt(long="--ignored-envs")]
    pub ignored_envs: Vec<String>,
}

impl Options {
    pub fn init_logging(&self) -> Result<(), log::SetLoggerError> {
        let level = match self.debug {
            true => log::LevelFilter::Debug,
            false => log::LevelFilter::Info,
        };

        let mut config_builder = simplelog::ConfigBuilder::new();

        simplelog::SimpleLogger::init(level, config_builder.build())
    }
}
