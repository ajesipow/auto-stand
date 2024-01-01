mod motor;
mod movement;
mod primitives;
mod sensor;
mod table;

use clap::Parser;
use clap::Subcommand;
use env_logger::Builder;
use log::LevelFilter;

use crate::movement::Movement;
use crate::primitives::Centimeter;
use crate::table::StandingDesk;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Commands,
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
}

fn main() {
    let cli = Cli::parse();
    let mut table = StandingDesk::new();

    let mut builder = Builder::new();

    let builder = match cli.debug {
        0 => builder.filter_level(LevelFilter::Error),
        1 => builder.filter_level(LevelFilter::Warn),
        2 => builder.filter_level(LevelFilter::Info),
        _ => builder.filter_level(LevelFilter::Debug),
    };
    builder.init();

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
                .move_to_height(Centimeter::new(height))
                .expect("moving to height to work");
        }
    }
}
