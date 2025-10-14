
use clap::Parser;
use colored::Colorize;
use embargo_toml::EmbargoFile;
use error::EmbargoResult;
use log::LevelFilter;
use commands::Commands;

use log::debug;

mod commands;
mod embargo_toml;
mod executions;
mod error;

static LOG_LEVEL_DEBUG: LevelFilter = LevelFilter::Trace;
static LOG_LEVEL_RELEASE: LevelFilter = LevelFilter::Off;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,

    /// Manually set the logging level
    #[arg(long)]
    debug_log: bool,
}

fn main() {
    match real_main() {
        Ok(msg) => {
            if let Some(msg) = msg {
                println!("{}", msg);
            }
        }
        Err(e) => eprintln!("{}: {}", "Error".red().bold(), e),
    }
}

fn real_main() -> EmbargoResult {

    use Commands::*;

    let args = Args::parse();

    let mut logger = env_logger::builder();

    if cfg!(debug_assertions) || args.debug_log {
        logger.filter_level(LOG_LEVEL_DEBUG);
    } else {
        logger.filter_level(LOG_LEVEL_RELEASE);
    }
    logger.init();

    let (embargo_toml, embargo_toml_path) = EmbargoFile::read_file()?;
    // embargo_toml_path.pop(); // make it so the path doesn't include the file itself
    debug!("Embargo.toml read: {:?}\nPath: {:?}", embargo_toml, embargo_toml_path);

    match args.command {

        Init => {
            debug!("Command executed: Init");
            // Todo - probably want to make some init args, like project type etc
            executions::create_project(None)
        },

        New(new_args) => {
            debug!("Command executed: New\nArgs: {:?}", new_args);
            executions::create_project(Some(new_args))
        },

        Build(build_args) => {
            debug!("Command executed: Build");
            executions::build_project(build_args, &embargo_toml, &embargo_toml_path)
        },
        
        Run(run_args) => {
            debug!("Command executed: Run");
            executions::run_project(run_args, &embargo_toml, &embargo_toml_path)
        },

        Clean(clean_args) => {
            debug!("Command executed: Clean");
            executions::clean_project(clean_args, &embargo_toml, &embargo_toml_path)
        },
    }
}
