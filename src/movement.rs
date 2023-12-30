use anyhow::Result;

use crate::primitives::Centimeter;

/// A trait for making movements.
pub(crate) trait Movement {
    /// Move to the standing position.
    fn move_to_standing(&mut self) -> Result<()>;

    /// Move to the sitting position.
    fn move_to_sitting(&mut self) -> Result<()>;

    /// Calibrates movements so that moving to a specific height is accurate.
    fn calibrate(&mut self) -> Result<()>;

    /// Moves to a specific height in centimeters.
    fn move_to_height(
        &self,
        height_cm: Centimeter,
    ) -> Result<()>;
}
