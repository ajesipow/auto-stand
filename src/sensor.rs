use anyhow::Result;

use crate::primitives::Centimeter;

pub(crate) trait Sensor {
    /// Takes a height measurement in centimeters.
    fn get_current_height(&self) -> Result<Centimeter>;

    /// Sets the lowest height as reference for calibration.
    fn set_min_height(
        &mut self,
        height: Centimeter,
    );

    /// Sets the highest height as reference for calibration.
    fn set_max_height(
        &mut self,
        height: Centimeter,
    );
}

pub(crate) struct DistanceSensor;

impl DistanceSensor {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl Sensor for DistanceSensor {
    fn get_current_height(&self) -> Result<Centimeter> {
        todo!()
    }

    fn set_min_height(
        &mut self,
        _height: Centimeter,
    ) {
        todo!()
    }

    fn set_max_height(
        &mut self,
        _height: Centimeter,
    ) {
        todo!()
    }
}
