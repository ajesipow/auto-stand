use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

use log::debug;
use rppal::gpio::Gpio;
use rppal::gpio::OutputPin;

use crate::config::MotorConfig;

/// A driver for handling the movement of the standing desk's motor.
pub(crate) trait MotorDriver {
    /// Makes the motor move the table up until the provided condition is false
    /// or until the timeoout is reached.
    fn up_until_false_or_timeout<F>(
        &mut self,
        condition: &mut F,
    ) where
        F: FnMut() -> bool;

    /// Makes the motor move the table up until the provided condition is false
    /// or until the timeoout is reached.
    fn down_until_false_or_timeout<F>(
        &mut self,
        condition: &mut F,
    ) where
        F: FnMut() -> bool;
}

// The standard struct for driving the desk motor.
#[derive(Debug)]
pub(crate) struct DeskMotorDriver {
    motor: DeskMotor,
    // The motor should not be (tried) to run for longer than this duration
    timeout: Duration,
    // A receiver for an issued shutdown signal. We need this to gracefully stop the motor and drop
    // reset the pins correctly.
    shutdown_rx: Receiver<()>,
}

impl DeskMotorDriver {
    /// Creates a new `DeskMotorDriver` with the provided configuration.
    ///
    /// The `shutdown_rx` receiver is used for gracefully stopping the motor.
    ///
    /// # Panics
    /// Panics if the configured pins for driving the motor up or down are the
    /// same or if they cannot be initialised.
    pub fn new(
        config: MotorConfig,
        shutdown_rx: Receiver<()>,
    ) -> Self {
        Self {
            motor: DeskMotor::new(config),
            timeout: Duration::from_secs(config.timeout_secs),
            shutdown_rx,
        }
    }

    fn move_until_false_or_timeout<C>(
        &mut self,
        direction: MoveDirection,
        condition: &mut C,
    ) where
        C: FnMut() -> bool,
    {
        let now = Instant::now();
        match direction {
            MoveDirection::Up => self.motor.up(),
            MoveDirection::Down => self.motor.down(),
        }
        while condition()
            && now.elapsed() < self.timeout
            && matches!(self.shutdown_rx.try_recv(), Err(TryRecvError::Empty))
        {
            sleep(Duration::from_millis(50));
        }
        self.motor.stop();
    }
}

impl MotorDriver for DeskMotorDriver {
    fn up_until_false_or_timeout<C>(
        &mut self,
        condition: &mut C,
    ) where
        C: FnMut() -> bool,
    {
        self.move_until_false_or_timeout(MoveDirection::Up, condition);
    }

    fn down_until_false_or_timeout<C>(
        &mut self,
        condition: &mut C,
    ) where
        C: FnMut() -> bool,
    {
        self.move_until_false_or_timeout(MoveDirection::Down, condition);
    }
}

#[derive(Debug)]

enum MoveDirection {
    Up,
    Down,
}

#[derive(Debug)]
struct DeskMotor {
    pin_up: OutputPin,
    pin_down: OutputPin,
}

impl DeskMotor {
    /// Creates a new `DeskMotor`.
    ///
    /// # Panics
    /// Panics if the configured pins for driving the motor up or down are the
    /// same or if they cannot be initialised.
    fn new(config: MotorConfig) -> Self {
        let gpio = Gpio::new().expect("gpio to be available");
        let pin_up = gpio
            .get(config.up_pin)
            .expect("pin up to be available")
            .into_output();
        let pin_down = gpio
            .get(config.down_pin)
            .expect("pin down to be available")
            .into_output();
        Self { pin_up, pin_down }
    }

    fn up(&mut self) {
        self.stop();
        debug!("Moving up");
        self.pin_up.set_high();
    }

    fn down(&mut self) {
        self.stop();
        debug!("Moving down");
        self.pin_down.set_high();
    }

    fn stop(&mut self) {
        debug!("Stopping");
        self.pin_up.set_low();
        self.pin_down.set_low();
    }
}
