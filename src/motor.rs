use std::time::SystemTime;

use log::debug;
use rppal::gpio::{Gpio, Pin};
use rppal::gpio::OutputPin;

use crate::config::MotorConfig;

/// A basic crate for the motor driving the standing desk.
pub(crate) trait Motor {
    /// Makes the motor move the table up until [`Motor::stop`] is called.
    fn up(&mut self);

    /// Makes the motor move the table down until [`Motor::stop`] is called.
    fn down(&mut self);

    /// Stops the motor.
    fn stop(&mut self);
}

pub(crate) struct DeskMotor {
    pin_up: Pin,
    pin_down: Pin,
    last_start_time: Option<SystemTime>,
}

impl DeskMotor {
    /// Creates a new DeskMotor.
    /// Panics if the configured pins for driving the motor up or down are the same or if they
    /// cannot be initialised.
    pub(crate) fn new(config: MotorConfig) -> Self {
        let gpio = Gpio::new().expect("gpio to be available");
        let pin_up = gpio
            .get(config.up_pin)
            .expect("pin up to be available");
        println!("pin up level and mode {:?}, {:?}", pin_up.read(), pin_up.mode());
        let pin_down = gpio
            .get(config.down_pin)
            .expect("pin down to be available");
        println!("pin down level and mode {:?}, {:?}", pin_down.read(), pin_down.mode());
        Self {
            pin_up,
            pin_down,
            last_start_time: None,
        }
    }
}

impl Motor for DeskMotor {
    fn up(&mut self) {
        self.stop();
        debug!("Moving up");
        // self.pin_up.into_output().set_high();
    }

    fn down(&mut self) {
        self.stop();
        debug!("Moving down");
        // self.pin_down.into_output().set_high();
    }

    fn stop(&mut self) {
        debug!("Stopping");
        // self.pin_up.into_output().set_low();
        // self.pin_down.into_output().set_low();
    }
}
