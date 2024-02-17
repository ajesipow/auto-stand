mod config;
mod motor;
mod movement;
mod primitives;
mod sensor;
mod table;

use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::thread::sleep;
use std::time::Duration;

use clap::Parser;
use clap::Subcommand;
use env_logger::Builder;
use log::LevelFilter;
use simple_signal::Signal;

use crate::config::Config;
use crate::movement::Movement;
use crate::primitives::Centimeter;
use crate::table::StandingDesk;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    /// The path to the config file
    #[arg(short, long, value_name = "FILE")]
    config: PathBuf,
}

#[derive(Subcommand)]
enum Commands {
    Calibrate,
    Sitting,
    Standing,
    #[command(arg_required_else_help = true)]
    MoveTo {
        height: u8,
    },
    TestSensor,
}

fn main() {
    let cli = Cli::parse();
    let config = Config::load(cli.config).expect("be able to load configuration");
    let (shutdown_tx, shutdown_rx) = channel::<()>();

    let mut builder = Builder::new();

    let builder = match cli.debug {
        0 => builder.filter_level(LevelFilter::Error),
        1 => builder.filter_level(LevelFilter::Warn),
        2 => builder.filter_level(LevelFilter::Info),
        _ => builder.filter_level(LevelFilter::Debug),
    };
    builder.init();

    simple_signal::set_handler(&[Signal::Int, Signal::Term], move |_| {
        println!("Shutting down");
        shutdown_tx
            .send(())
            .expect("be able to send a shutdown signal")
    });

    let mut table = StandingDesk::new(config, shutdown_rx);
    match cli.command {
        Commands::Calibrate => {
            table.calibrate().expect("calibration to work");
        }
        Commands::Sitting => {
            table
                .move_to_sitting()
                .expect("moving to sitting position to work");
        }
        Commands::Standing => {
            table
                .move_to_standing()
                .expect("moving to standing position to work");
        }
        Commands::MoveTo { height } => {
            println!("Moving to height {:?} ...", height);
            table
                .move_to_height(Centimeter(height))
                .expect("moving to height to work");
        }
        Commands::TestSensor => {
            println!("Testing distance sensor");
            let mut i = 0;
            while i < 50 {
                sleep(Duration::from_millis(200));
                let current_height = table.get_measurement().unwrap().0;
                println!("current distance: {current_height:?}");
                i += 1;
            }
        }
    };
}
