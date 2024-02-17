use std::fs;
use std::sync::mpsc::Receiver;

use anyhow::anyhow;
use anyhow::Result;
use log::debug;
use log::info;

use crate::config::Config;
use crate::config::TableConfig;
use crate::motor::DeskMotorDriver;
use crate::motor::MotorDriver;
use crate::movement::Movement;
use crate::primitives::Centimeter;
use crate::sensor::DistanceSensor;
use crate::sensor::HCSR04;

/// The standing desk implementation.
#[derive(Debug)]
pub(crate) struct StandingDesk<S: DistanceSensor = HCSR04, M: MotorDriver = DeskMotorDriver> {
    config: TableConfig,
    sensor: S,
    motor_driver: M,
}

impl StandingDesk {
    /// Creates a new instance of a standing desk.
    pub fn new(
        config: Config,
        shutdown_rx: Receiver<()>,
    ) -> Self {
        let sensor = HCSR04::new(config.sensor);
        let motor_driver = DeskMotorDriver::new(config.motor, shutdown_rx);
        Self {
            config: config.table,
            sensor,
            motor_driver,
        }
    }

    pub fn get_measurement(&mut self) -> Result<Centimeter> {
        self.sensor.current_height()
    }
}

impl<S: DistanceSensor, M: MotorDriver> Movement for StandingDesk<S, M> {
    fn move_to_standing(&mut self) -> Result<()> {
        info!("Moving to standing position ...");
        self.move_to_height(self.config.standing_height)
    }

    fn move_to_sitting(&mut self) -> Result<()> {
        info!("Moving to standing position ...");
        self.move_to_height(self.config.sitting_height)
    }

    fn calibrate(&mut self) -> Result<()> {
        info!("Calibrating");
        self.motor_driver.up_until_false_or_timeout(&mut || true);
        self.sensor.set_max_height(self.config.max_table_height)?;

        self.motor_driver.down_until_false_or_timeout(&mut || true);
        self.sensor.set_min_height(self.config.min_table_height)?;

        let calibration_file = self.sensor.calibration_file();
        let raw_calibration_data = toml::to_string(&self.sensor.calibration_data())?;
        fs::write(calibration_file, raw_calibration_data)?;
        debug!("Calibration data written to {calibration_file:?}");

        // self.move_to_sitting()
        Ok(())
    }

    fn move_to_height(
        &mut self,
        height_cm: Centimeter,
    ) -> Result<()> {
        if height_cm > self.config.max_table_height {
            return Err(anyhow!(
                "Cannot move table higher than {:?}",
                self.config.max_table_height
            ));
        } else if height_cm < self.config.min_table_height {
            return Err(anyhow!(
                "Cannot move table lower than {:?}",
                self.config.min_table_height
            ));
        }
        info!("Moving to height {height_cm:?}");
        let current_height = self.sensor.current_height()?;
        // We allow for some tolerance as moving the table is not so precise
        if height_cm - Centimeter(1) <= current_height
            && current_height <= height_cm + Centimeter(1)
        {
            debug!("Table already at desired height");
            return Ok(());
        }
        if current_height < height_cm {
            self.motor_driver
                // FIXME do not use unwrap
                .up_until_false_or_timeout(&mut || {
                    self.sensor.current_height().unwrap() < height_cm
                });
        }
        if current_height > height_cm {
            self.motor_driver
                // FIXME do not use unwrap
                .down_until_false_or_timeout(&mut || {
                    self.sensor.current_height().unwrap() > height_cm
                });
        }
        Ok(())
    }
}
