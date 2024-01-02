use std::ops::Add;
use std::ops::Sub;

/// A reasonable unit for table height.
/// The table cannot be higher than 255cm
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) struct Centimeter(pub u8);

impl Centimeter {
    pub(crate) fn new(centimeter: u8) -> Self {
        Self(centimeter)
    }

    pub(crate) fn into_inner(self) -> u8 {
        self.0
    }
}

impl Sub for Centimeter {
    type Output = Self;

    fn sub(
        self,
        rhs: Self,
    ) -> Self::Output {
        Centimeter(self.0 - rhs.0)
    }
}

impl Add for Centimeter {
    type Output = Self;

    fn add(
        self,
        rhs: Self,
    ) -> Self::Output {
        Centimeter(self.0 + rhs.0)
    }
}
