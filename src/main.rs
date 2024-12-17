
use clap::Parser;
use executions::BuildFlags;
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

    use Commands::*;

    let mut logger = env_logger::builder();

    if cfg!(debug_assertions) {
        logger.filter_level(LOG_LEVEL);
    }
    logger.init();

    let args = Args::parse();
    // debug!("Arguments: {:?}", args);
    let t = std::env::var("EMBARGO_HOME");
    println!("{:?}", t);

    let result = match args.command {

        Init => {
            debug!("Command executed: Init");
            // Todo - probably want to make some init args, like project type etc
            executions::create_project(None)
        },

        New(new_args) => {
            debug!("Command executed: New\nArgs: {:?}", new_args);
            executions::create_project(Some(new_args))
        },

        Build => {
            debug!("Command executed: Build");
            let b = BuildFlags{};
            executions::build_project(b)
        },
        
        Run => {
            debug!("Command executed: Run");
            unimplemented!()
        }
        _ => unimplemented!(),
    };

    match result {
        Ok(msg) => println!("{}", msg),
        Err(e) => eprintln!("An error has occurred: {}", e),
    }
}
