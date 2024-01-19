use std::env;
use std::path::Path;

use clap::Parser;
use log::{error, info};

use sweeper::{IdeaSweeper, MavenSweeper, SweeperChain};

mod argument;
mod sweeper;

fn init_logger(debug: bool) {
    let mut log_builder = env_logger::builder();
    if debug {
        log_builder.parse_filters("debug").init();
    } else {
        log_builder.parse_filters("info").init();
    }
}

#[tokio::main]
async fn main() {
    let arg = argument::Argument::parse();
    init_logger(arg.debug);

    let path = Path::new(&arg.path).to_path_buf();
    let path = if path.is_relative() {
        let current_dir = env::current_dir().unwrap();
        current_dir.join(path)
    } else {
        path
    };

    info!("Clear Project: {}", path.display());

    if !arg.maven && !arg.idea {
        return;
    }

    let mut sweeper_chain = SweeperChain::new();

    if arg.maven {
        let sweeper: Box<MavenSweeper> = Box::default();
        sweeper_chain.add_sweeper(sweeper);
    }

    if arg.idea {
        let sweeper: Box<IdeaSweeper> = Box::default();
        sweeper_chain.add_sweeper(sweeper);
    }

    if let Err(e) = sweeper_chain.clean(&path) {
        error!("Failed to clean the project file: {}", e.to_string());
    }
}
