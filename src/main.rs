use clap::Parser;
use colored::Colorize;
use embargo::{cli::Args, run};
use log::LevelFilter;

static LOG_LEVEL_DEBUG: LevelFilter = LevelFilter::Trace;
static LOG_LEVEL_RELEASE: LevelFilter = LevelFilter::Off;

fn main() {
    let args = Args::parse();
    let mut logger = env_logger::builder();
    if cfg!(debug_assertions) || args.debug_log() {
        logger.filter_level(LOG_LEVEL_DEBUG);
    } else {
        logger.filter_level(LOG_LEVEL_RELEASE);
    }
    logger.init();
    
    match run(args, None) {
        Ok(msg) => {
            if let Some(msg) = msg {
                println!("{}", msg);
            }
        }
        Err(e) => eprintln!("{}: {}", "Error".red().bold(), e),
    }
}