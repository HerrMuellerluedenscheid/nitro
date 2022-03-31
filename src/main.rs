#![feature(generators)]
#[macro_use]
extern crate log;

use std::path::Path;
use std::process;
use std::time::{Duration, Instant};

use clap::Parser;
use env_logger::Builder;
use log::LevelFilter;
use tokio::time;

use crate::authentication_logs::watch_authentication_logs;
use crate::hashing::init_hash_db;

use crate::config::{load_config_from_file, print_basic_config};
use crate::persist::check_files;

mod authentication_logs;
mod config;
mod hashing;
mod notifiers;
mod persist;

static DB_DIRECTORY: &str = "/tmp/nitros.db";
static DEFAULT_CONFIG: &str = "/etc/nitro/config.yaml";
static TIMEOUT: u64 = 1000;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// print a demo configuration
    #[clap(long)]
    demo_config: bool,

    /// initialize the database
    #[clap(short, long)]
    init: bool,

    /// Start scanning files
    #[clap(short, long)]
    scan: bool,

    /// Start scanning authentication
    #[clap(short, long)]
    watch_authentication: bool,
}

#[tokio::main]
async fn main() {
    Builder::new().filter_level(LevelFilter::Info).init();
    let args = Cli::parse();

    if args.demo_config == true {
        print_basic_config();
        process::exit(0);
    }
    let config = load_config_from_file(Path::new(DEFAULT_CONFIG)).unwrap();

    let start = Instant::now();
    if args.init == true {
        init_hash_db(config).await;
    } else if args.scan == true {
        check_files(config).await;
    } else if args.watch_authentication {
        let test_file = Path::new("/tmp/auth.log");
        watch_authentication_logs(&test_file).await;
    }
    info!("Time elapsed to hash: {:?}", start.elapsed());

    info!("Waiting a second for dispatcher to complete");
    time::sleep(Duration::from_millis(TIMEOUT)).await;
}
