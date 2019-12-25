use crate::epd4in3::{DEFAULT_BACKGROUND_COLOR, HEIGHT, WIDTH};
use crate::graphics::{Display, DisplayRotation};
use crate::prelude::*;
use embedded_graphics::prelude::*;

/// Full size buffer for use with the 4in3 EPD
///
/// Can also be manuall constructed:
/// `buffer: [DEFAULT_BACKGROUND_COLOR.get_byte_value(); WIDTH / 8 * HEIGHT]`
pub struct Display4in3 {
    buffer: [EpdColor; WIDTH as usize * HEIGHT as usize],
    rotation: DisplayRotation,
}

impl Default for Display4in3 {
    fn default() -> Self {
        Display4in3 {
            buffer: [DEFAULT_BACKGROUND_COLOR; WIDTH as usize * HEIGHT as usize],
            rotation: DisplayRotation::default(),
        }
    }
}

impl Drawing<EpdColor> for Display4in3 {
    fn draw<T>(&mut self, item_pixels: T)
    where
        T: IntoIterator<Item = Pixel<EpdColor>>,
    {
        self.draw_helper(WIDTH, HEIGHT, item_pixels);
    }
}

impl Display for Display4in3 {
    fn buffer(&self) -> &[EpdColor] {
        &self.buffer
    }

    fn get_mut_buffer(&mut self) -> &mut [EpdColor] {
        &mut self.buffer
    }

    fn set_rotation(&mut self, rotation: DisplayRotation) {
        self.rotation = rotation;
    }

    fn rotation(&self) -> DisplayRotation {
        self.rotation
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::EpdColor;
    use crate::epd4in3;
    use crate::graphics::{Display, DisplayRotation};
    use embedded_graphics::geometry::Point;
    use embedded_graphics::primitives::Line;

    // test buffer length
    #[test]
    fn graphics_size() {
        let display = Display4in3::default();
        assert_eq!(display.buffer().len(), 480000);
    }

    // test default background color on all bytes
    #[test]
    fn graphics_default() {
        let display = Display4in3::default();
        for &byte in display.buffer() {
            assert_eq!(byte, epd4in3::DEFAULT_BACKGROUND_COLOR);
        }
    }

    #[test]
    fn graphics_rotation_0() {
        let mut display = Display4in3::default();
        display.draw(
            Line::new(Point::new(0, 0), Point::new(7, 0))
                .stroke(Some(EpdColor::Black))
                .into_iter(),
        );

        let buffer = display.buffer();

        assert_eq!(buffer[0], EpdColor::Black);

        for &byte in buffer.iter().skip(8) {
            assert_eq!(byte, epd4in3::DEFAULT_BACKGROUND_COLOR);
        }
    }

    #[test]
    fn graphics_rotation_180() {
        let mut display = Display4in3::default();
        display.set_rotation(DisplayRotation::Rotate180);
        display.draw(
            Line::new(Point::new(792, 599), Point::new(799, 599))
                .stroke(Some(EpdColor::Black))
                .into_iter(),
        );

        let buffer = display.buffer();

        //extern crate std;
        //std::println!("{:?}", buffer);

        assert_eq!(buffer[0], EpdColor::Black);

        for &byte in buffer.iter().skip(8) {
            assert_eq!(byte, epd4in3::DEFAULT_BACKGROUND_COLOR);
        }
    }
}
