use rppal::gpio::Gpio;
use rppal::gpio::OutputPin;

/// The pin number controlling the motor's upwards movement
const PIN_UP: u8 = 23;

/// The pin number controlling the motor's downwards movement
const PIN_DOWN: u8 = 24;

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
    pin_up: OutputPin,
    pin_down: OutputPin,
}

impl DeskMotor {
    /// Creates a new DeskMotor.
    /// Panics if the configured pins for driving the motor up or down are the same or if they
    /// cannot be initialised.
    pub(crate) fn new() -> Self {
        let gpio = Gpio::new().expect("gpio to be available");
        let pin_up = gpio
            .get(PIN_UP)
            .expect("pin up to be available")
            .into_output();
        let pin_down = gpio
            .get(PIN_DOWN)
            .expect("pin down to be available")
            .into_output();
        Self { pin_up, pin_down }
    }
}

impl Motor for DeskMotor {
    fn up(&mut self) {
        self.stop();
        self.pin_up.set_high();
    }

    fn down(&mut self) {
        self.stop();
        self.pin_down.set_high();
    }

    fn stop(&mut self) {
        self.pin_up.set_low();
        self.pin_down.set_low();
    }
}
