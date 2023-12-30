/// A reasonable unit for table height.
/// The table cannot be higher than 255cm
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) struct Centimeter(u8);

impl Centimeter {
    pub(crate) fn new(centimeter: u8) -> Self {
        Self(centimeter)
    }
}
