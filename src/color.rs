//! B/W/G/DG Color for EPDs

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EpdColor {
    Black = 0,
    DarkGray = 1,
    Gray = 2,
    White = 3,
}

impl EpdColor {
    /// Get the color encoding of the color for one bit
    pub fn get_bit_value(self) -> u8 {
        match self {
            EpdColor::White => 3u8,
            EpdColor::Black => 0u8,
            EpdColor::DarkGray => 1u8,
            EpdColor::Gray => 2u8,
        }
    }

    /// Gets a full byte of black or white pixels
    pub fn get_byte_value(self) -> u8 {
        match self {
            EpdColor::White => 0xff,
            EpdColor::Black => 0x00,
            EpdColor::DarkGray => 0x55,
            EpdColor::Gray => 0xaa,
        }
    }

    /// Parses from u8 to Color
    fn from_u8(val: u8) -> Self {
        match val {
            0 => EpdColor::Black,
            1 => EpdColor::DarkGray,
            2 => EpdColor::Gray,
            3 => EpdColor::White,
            e => panic!(
                "DisplayColor only parses 0 and 3 (Black and White) and not `{}`",
                e
            ),
        }
    }

    /// Returns the inverse of the given color.
    ///
    /// Black returns White and White returns Black
    pub fn inverse(self) -> EpdColor {
        match self {
            EpdColor::White => EpdColor::Black,
            EpdColor::Black => EpdColor::White,
            EpdColor::Gray => EpdColor::DarkGray,
            EpdColor::DarkGray => EpdColor::Gray,
        }
    }
}

#[cfg(feature = "graphics")]
use embedded_graphics::prelude::PixelColor;
#[cfg(feature = "graphics")]
impl PixelColor for EpdColor {
    type Raw = ();
}

impl From<u8> for EpdColor {
    fn from(value: u8) -> Self {
        EpdColor::from_u8(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_u8() {
        assert_eq!(EpdColor::Black, EpdColor::from(0u8));
        assert_eq!(EpdColor::White, EpdColor::from(3u8));
        assert_eq!(EpdColor::Gray, EpdColor::from(2u8));
        assert_eq!(EpdColor::DarkGray, EpdColor::from(1u8));
    }

    // test all values aside from 0,1,2 and 3 which all should panic
    #[test]
    fn from_u8_panic() {
        for val in 4..=u8::max_value() {
            extern crate std;
            let result = std::panic::catch_unwind(|| EpdColor::from(val));
            assert!(result.is_err());
        }
    }

    #[test]
    fn u8_conversion_black() {
        assert_eq!(
            EpdColor::from(EpdColor::Black.get_bit_value()),
            EpdColor::Black
        );
        assert_eq!(EpdColor::from(0u8).get_bit_value(), 0u8);
    }

    #[test]
    fn u8_conversion_white() {
        assert_eq!(
            EpdColor::from(EpdColor::White.get_bit_value()),
            EpdColor::White
        );
        assert_eq!(EpdColor::from(3u8).get_bit_value(), 3u8);
    }
}
