
use clap::Parser;
use embargo_toml::{EmbargoFile, GlobalEmbargoFile};
use error::EmbargoResult;
use log::LevelFilter;
use commands::Commands;

use log::debug;

mod commands;
mod embargo_toml;
mod executions;
mod error;

static LOG_LEVEL: LevelFilter = LevelFilter::Debug;
// static LOG_LEVEL: LevelFilter = LevelFilter::Off;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    match real_main() {
        Ok(msg) => {
            if let Some(msg) = msg {
                println!("{}", msg);
            }
        }
        Err(e) => eprintln!("An error has occurred: {}", e),
    }
}

fn real_main() -> EmbargoResult {

    use Commands::*;

    let mut logger = env_logger::builder();

    if cfg!(debug_assertions) {
        logger.filter_level(LOG_LEVEL);
    }
    logger.init();

    let args = Args::parse();
   
    let global_file = GlobalEmbargoFile::try_read()?;

    let (embargo_toml, embargo_toml_path) = EmbargoFile::read_file()?;
    debug!("Embargo.toml read: {:?}\nPath: {}", embargo_toml, embargo_toml_path.display());

    return match args.command {

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
            executions::build_project(build_args, &global_file, &embargo_toml)
        },
        
        Run(run_args) => {
            debug!("Command executed: Run");
            executions::run_project(run_args, &global_file, &embargo_toml, &embargo_toml_path)
        }
        _ => unimplemented!(),
    }
}
