mod motor;
mod movement;
mod primitives;
mod sensor;
mod table;

use clap::Parser;

use crate::movement::Movement;
use crate::primitives::Centimeter;
use crate::table::StandingDesk;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
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
    let cli = Commands::parse();
    let mut table = StandingDesk::new();

    match cli {
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
