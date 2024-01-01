use std::thread::sleep;
use std::time::Duration;

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
static MAX_HEIGHT: Lazy<Centimeter> = Lazy::new(|| Centimeter::new(150));
static STANDING_HEIGHT: Lazy<Centimeter> = Lazy::new(|| Centimeter::new(110));
static SITTING_HEIGHT: Lazy<Centimeter> = Lazy::new(|| Centimeter::new(60));
static MIN_HEIGHT: Lazy<Centimeter> = Lazy::new(|| Centimeter::new(40));

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
        let mut previous_height = current_height;
        // TODO add tolerance?
        // TODO add timeout
        while current_height != previous_height {
            // Table is still moving up
            sleep(Duration::from_millis(200));
            previous_height = current_height;
            current_height = self.sensor.get_current_height()?;
        }
        self.motor.stop();
        self.sensor.set_max_height(self.max_height)?;

        self.motor.down();
        let mut current_height = self.sensor.get_current_height()?;
        let mut previous_height = current_height;
        // TODO add tolerance?
        // TODO add timeout
        while current_height != previous_height {
            // Table is still moving up
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
        info!("Moving to height {height_cm:?}");
        let current_height = self.sensor.get_current_height()?;
        if current_height == height_cm {
            debug!("Table already at desired height");
            return Ok(());
        }
        // TODO add some tolerance
        // TODO add timeout
        if current_height < height_cm {
            self.motor.up();
            while self.sensor.get_current_height()? < height_cm {
                sleep(Duration::from_millis(200));
            }
            self.motor.stop();
        }
        // TODO add some tolerance
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
