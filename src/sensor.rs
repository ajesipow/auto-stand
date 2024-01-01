use std::thread::sleep;
use std::time::Duration;
use std::time::SystemTime;

use anyhow::anyhow;
use anyhow::Result;
use rppal::gpio::Gpio;
use rppal::gpio::InputPin;
use rppal::gpio::OutputPin;
use rppal::gpio::Trigger;

use crate::primitives::Centimeter;

/// The pin number controlling the distance sensor's trigger
const TRIGGER_PIN: u8 = 4;

/// The pin number listening for the distance sensor's echo signal
const ECHO_PIN: u8 = 17;

pub(crate) trait DistanceSensor {
    /// Takes a height measurement in centimeters.
    fn get_current_height(&mut self) -> Result<Centimeter>;

    /// Sets the lowest height as reference for calibration.
    fn set_min_height(
        &mut self,
        height: Centimeter,
    ) -> Result<()>;

    /// Sets the highest height as reference for calibration.
    fn set_max_height(
        &mut self,
        height: Centimeter,
    ) -> Result<()>;
}

/// The HCSR04 sensor for measuring distances.
pub(crate) struct HCSR04 {
    calibration_data: SensorCalibrationData,
    trigger_pin: OutputPin,
    echo_pin: InputPin,
}

struct SensorCalibrationData {
    // The minimum height we can observe
    min_height: Centimeter,
    // The duration of the echo in ms at minimum height
    min_height_echo: Duration,
    // The max height we can observe
    max_height: Centimeter,
    // The duration of the echo in ms at max height
    max_height_echo: Duration,
}

impl HCSR04 {
    /// Creates a new [HCSR04] with predefined calibration parameters.
    pub(crate) fn new() -> Self {
        // TODO read calibration data from file
        let gpio = Gpio::new().expect("gpio to be available");
        Self {
            calibration_data: SensorCalibrationData {
                min_height: Centimeter::new(40),
                min_height_echo: Duration::from_micros(1166),
                max_height: Centimeter::new(130),
                max_height_echo: Duration::from_micros(3790),
            },
            trigger_pin: gpio
                .get(TRIGGER_PIN)
                .expect("trigger pin be available")
                .into_output(),
            echo_pin: gpio
                .get(ECHO_PIN)
                .expect("echo pin be available")
                // Echo should be on low per default
                .into_input_pulldown(),
        }
    }

    /// Measures the time it takes for the sensor to send and receive an acoustic echo.
    /// # Errors
    /// Errors if there is no object close enough or the object is too small.
    fn measure_full_echo_duration(&mut self) -> Result<Duration> {
        // We want to block on both rising and falling signal edges which indicate the start and end
        // of a measurement respectively.
        // TODO move this into the constructor?
        self.echo_pin.set_interrupt(Trigger::Both)?;

        // TODO add mechanism to prevent too frequent calls of this function
        self.trigger_pin.set_high();
        // Trigger needs to be set to high for at least 10us, let's be certain here with 100us.
        sleep(Duration::from_micros(100));
        // A falling signal edge is the actual trigger for the sensor to start the measurement.
        self.trigger_pin.set_low();
        // Wait for the rising edge indicating the start of the measurement.
        // We expect a delay of around 500us as per the datasheet:
        // https://www.mikrocontroller.net/attachment/218122/HC-SR04_ultraschallmodul_beschreibung_3.pdf
        self.echo_pin
            .poll_interrupt(true, Some(Duration::from_millis(1)))?;
        let start_time = SystemTime::now();
        // Let's wait for the falling edge indicating the end of the measurement.
        // No need to reset the interrupt as we've just received the last event.
        // Timeout is 250ms as the sensor should return to low after 200ms max to indicate an
        // unsuccessful measurement.
        self.echo_pin
            .poll_interrupt(false, Some(Duration::from_millis(250)))?;
        let echo_duration = start_time.elapsed()?;
        if echo_duration >= Duration::from_millis(200) {
            return Err(anyhow!("unsuccessful measurement"));
        }
        Ok(echo_duration)
    }
}

impl DistanceSensor for HCSR04 {
    fn get_current_height(&mut self) -> Result<Centimeter> {
        let echo_duration = self.measure_full_echo_duration()?;
        // We're interpolating the height from our calibration parameters
        let normalized_echo = (echo_duration - self.calibration_data.min_height_echo).as_micros()
            / (self.calibration_data.max_height_echo - self.calibration_data.min_height_echo)
                .as_micros();
        Ok(
            normalized_echo * (self.calibration_data.max_height - self.calibration_data.min_height)
                + self.calibration_data.min_height,
        )
    }

    fn set_min_height(
        &mut self,
        height: Centimeter,
    ) -> Result<()> {
        // Make a measurement to get the timing
        // Set min height and min echo time
        self.calibration_data.min_height_echo = self.measure_full_echo_duration()?;
        self.calibration_data.min_height = height;
        Ok(())
    }

    fn set_max_height(
        &mut self,
        height: Centimeter,
    ) -> Result<()> {
        // Make a measurement to get the timing
        // Set min height and min echo time
        self.calibration_data.max_height_echo = self.measure_full_echo_duration()?;
        self.calibration_data.max_height = height;
        Ok(())
    }
}
