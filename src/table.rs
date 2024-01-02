use std::thread::sleep;
use std::time::Duration;

use anyhow::anyhow;
use anyhow::Result;
use log::debug;
use log::info;
use once_cell::sync::Lazy;

use crate::motor::DeskMotor;
use crate::motor::Motor;
use crate::movement::Movement;
use crate::primitives::Centimeter;
use crate::sensor::DistanceSensor;
use crate::sensor::HCSR04;

// TODO make configurable
static MAX_HEIGHT: Lazy<Centimeter> = Lazy::new(|| Centimeter(150));
static STANDING_HEIGHT: Lazy<Centimeter> = Lazy::new(|| Centimeter(110));
static SITTING_HEIGHT: Lazy<Centimeter> = Lazy::new(|| Centimeter(60));
static MIN_HEIGHT: Lazy<Centimeter> = Lazy::new(|| Centimeter(40));

/// The standing desk implementation.
#[derive(Debug)]
pub(crate) struct StandingDesk<S: DistanceSensor = HCSR04, M: Motor = DeskMotor> {
    max_height: Centimeter,
    min_height: Centimeter,
    sensor: S,
    motor: M,
}

impl StandingDesk {
    /// Creates a new instance of a standing desk.
    pub fn new() -> Self {
        let sensor = HCSR04::new();
        let motor = DeskMotor::new();
        Self {
            max_height: *MAX_HEIGHT,
            min_height: *MIN_HEIGHT,
            sensor,
            motor,
        }
    }
}

impl<S: DistanceSensor, M: Motor> Movement for StandingDesk<S, M> {
    fn move_to_standing(&mut self) -> Result<()> {
        info!("Moving to standing position ...");
        self.move_to_height(*STANDING_HEIGHT)
    }

    fn move_to_sitting(&mut self) -> Result<()> {
        info!("Moving to standing position ...");
        self.move_to_height(*SITTING_HEIGHT)
    }

    fn calibrate(&mut self) -> Result<()> {
        info!("Calibrating");
        self.motor.up();
        let mut current_height = self.sensor.get_current_height()?;
        // We subtract a bit to kick-start the while loop below
        let mut previous_height = current_height - Centimeter(1);
        // TODO add timeout
        while previous_height < current_height {
            // Table is still moving
            sleep(Duration::from_millis(200));
            previous_height = current_height;
            current_height = self.sensor.get_current_height()?;
        }
        self.motor.stop();
        self.sensor.set_max_height(self.max_height)?;

        self.motor.down();
        // TODO add timeout
        // We add a bit to kick-start the while loop below
        previous_height = current_height + Centimeter(1);
        while previous_height > current_height {
            // Table is still moving down
            sleep(Duration::from_millis(200));
            previous_height = current_height;
            current_height = self.sensor.get_current_height()?;
        }
        self.motor.stop();
        self.sensor.set_min_height(self.min_height)?;

        // TODO save calibration data to file

        self.move_to_sitting()
    }

    fn move_to_height(
        &mut self,
        height_cm: Centimeter,
    ) -> Result<()> {
        if height_cm > *MAX_HEIGHT {
            return Err(anyhow!("Cannot move table higher than {MAX_HEIGHT:?}"));
        } else if height_cm < *MIN_HEIGHT {
            return Err(anyhow!("Cannot move table lower than {MIN_HEIGHT:?}"));
        }
        info!("Moving to height {height_cm:?}");
        let current_height = self.sensor.get_current_height()?;
        // We allow for some tolerance as moving the table is not so precise
        if height_cm - Centimeter(1) <= current_height
            && current_height <= height_cm + Centimeter(1)
        {
            debug!("Table already at desired height");
            return Ok(());
        }
        // TODO add timeout
        if current_height < height_cm {
            self.motor.up();
            while self.sensor.get_current_height()? < height_cm {
                sleep(Duration::from_millis(200));
            }
            self.motor.stop();
        }
        // TODO add timeout
        if current_height > height_cm {
            self.motor.down();
            while self.sensor.get_current_height()? > height_cm {
                sleep(Duration::from_millis(200));
            }
            self.motor.stop();
        }
        Ok(())
    }
}
