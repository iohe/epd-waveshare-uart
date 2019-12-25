//! Graphics Support for EPDs

use crate::color::EpdColor;
use embedded_graphics::prelude::*;

/// DisplayRotation
#[derive(Clone, Copy)]
pub enum DisplayRotation {
    /// No rotation
    Rotate0,
    /// Rotate by 90 degrees clockwise
    Rotate90,
    /// Rotate by 180 degrees clockwise
    Rotate180,
    /// Rotate 270 degrees clockwise
    Rotate270,
}

impl Default for DisplayRotation {
    fn default() -> Self {
        DisplayRotation::Rotate0
    }
}

pub trait Display: Drawing<EpdColor> {
    /// Clears the buffer of the display with the chosen background color
    fn clear_buffer(&mut self, background_color: EpdColor) {
        for elem in self.get_mut_buffer().iter_mut() {
            *elem = background_color;
        }
    }

    /// Returns the buffer
    fn buffer(&self) -> &[EpdColor];

    /// Returns a mutable buffer
    fn get_mut_buffer(&mut self) -> &mut [EpdColor];

    /// Sets the rotation of the display
    fn set_rotation(&mut self, rotation: DisplayRotation);

    /// Get the current rotation of the display
    fn rotation(&self) -> DisplayRotation;

    /// Helperfunction for the Embedded Graphics draw trait
    ///
    /// Becomes uneccesary when const_generics become stablised
    fn draw_helper<T>(&mut self, width: u32, height: u32, item_pixels: T)
    where
        T: IntoIterator<Item = Pixel<EpdColor>>,
    {
        let rotation = self.rotation();
        let buffer = self.get_mut_buffer();
        for Pixel(point, color) in item_pixels {
            if outside_display(point.x as u32, point.y as u32, width, height, rotation) {
                continue;
            }

            // Give us index inside the buffer and the bit-position in that u8 which needs to be changed
            let index = find_position(point.x as u32, point.y as u32, width, height, rotation);
            buffer[index] = color;
        }
    }
}

/// A variable Display without a predefined buffer
///
/// The buffer can be created as following:
/// buffer: [DEFAULT_BACKGROUND_COLOR; WIDTH * HEIGHT]
///
/// Example:
/// ```rust,no_run
/// # use epd_waveshare_uart::epd4in3::DEFAULT_BACKGROUND_COLOR;
/// # use epd_waveshare_uart::prelude::*;
/// # use epd_waveshare_uart::graphics::VarDisplay;
/// # use embedded_graphics::prelude::*;
/// # use embedded_graphics::primitives::{Circle, Line};
/// let width = 296;
/// let height = 128;
///
/// let mut buffer = [DEFAULT_BACKGROUND_COLOR; 128 * 296];
/// let mut display = VarDisplay::new(width, height, &mut buffer);
///
/// display.set_rotation(DisplayRotation::Rotate90);
///
/// display.draw(
///     Line::new(Point::new(0, 120), Point::new(0, 295))
///         .stroke(Some(EpdColor::Black))
///         .into_iter(),
/// );
/// ```
pub struct VarDisplay<'a> {
    width: u32,
    height: u32,
    rotation: DisplayRotation,
    buffer: &'a mut [EpdColor],
}

impl<'a> VarDisplay<'a> {
    pub fn new(width: u32, height: u32, buffer: &'a mut [EpdColor]) -> VarDisplay<'a> {
        let len = buffer.len() as u32;
        assert!(width * height >= len);
        VarDisplay {
            width,
            height,
            rotation: DisplayRotation::default(),
            buffer,
        }
    }
}

impl<'a> Drawing<EpdColor> for VarDisplay<'a> {
    fn draw<T>(&mut self, item_pixels: T)
    where
        T: IntoIterator<Item = Pixel<EpdColor>>,
    {
        self.draw_helper(self.width, self.height, item_pixels);
    }
}

impl<'a> Display for VarDisplay<'a> {
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

// Checks if a pos is outside the defined display
fn outside_display(x: u32, y: u32, width: u32, height: u32, rotation: DisplayRotation) -> bool {
    match rotation {
        DisplayRotation::Rotate0 | DisplayRotation::Rotate180 => {
            if x >= width || y >= height {
                return true;
            }
        }
        DisplayRotation::Rotate90 | DisplayRotation::Rotate270 => {
            if y >= width || x >= height {
                return true;
            }
        }
    }
    false
}

#[rustfmt::skip]
//returns index position in the u8-slice and the bit-position inside that u8
fn find_position(x: u32, y: u32, width: u32, height: u32, rotation: DisplayRotation) -> usize {
    match rotation {
        DisplayRotation::Rotate0 => 
            (x  + (width ) * y) as usize,
        
        DisplayRotation::Rotate90 =>
            ((width - 1 - y)  + width  * x) as usize,
        
        DisplayRotation::Rotate180 => 
            ((width  * height - 1) - (x  + (width  * y))) as usize,
        
        DisplayRotation::Rotate270 => 
            (y  + height - 1 - (x * width)) as usize,
        
    }
}

#[cfg(test)]
mod tests {
    use super::{find_position, outside_display, Display, DisplayRotation, VarDisplay};
    use crate::color::EpdColor;
    use embedded_graphics::geometry::Point;
    use embedded_graphics::prelude::*;
    use embedded_graphics::primitives::Line;

    #[test]
    fn buffer_clear() {
        use crate::epd4in3::{HEIGHT, WIDTH};

        let mut buffer = [EpdColor::Black; WIDTH as usize * HEIGHT as usize];
        let mut display = VarDisplay::new(WIDTH, HEIGHT, &mut buffer);

        for &byte in display.buffer.iter() {
            assert_eq!(byte, EpdColor::Black);
        }

        display.clear_buffer(EpdColor::Gray);

        for &byte in display.buffer.iter() {
            assert_eq!(byte, EpdColor::Gray);
        }
    }

    #[test]
    fn rotation_overflow() {
        use crate::epd4in3::{HEIGHT, WIDTH};
        let width = WIDTH as u32;
        let height = HEIGHT as u32;
        test_rotation_overflow(width, height, DisplayRotation::Rotate0);    
    }

    fn test_rotation_overflow(width: u32, height: u32, rotation2: DisplayRotation) {
        let max_value = width * height;
        for x in 0..(width + height) {
            //limit x because it runs too long
            for y in 0..(u32::max_value()) {
                if outside_display(x, y, width, height, rotation2) {
                    break;
                } else {
                    let idx = find_position(x, y, width, height, rotation2);
                    assert!(idx < max_value as usize);
                }
            }
        }
    }

    #[test]
    fn graphics_rotation_0() {
        use crate::epd4in3::DEFAULT_BACKGROUND_COLOR;
        let width = 296;
        let height = 128;

        let mut buffer = [DEFAULT_BACKGROUND_COLOR; 128 * 296];
        let mut display = VarDisplay::new(width, height, &mut buffer);

        display.draw(
            Line::new(Point::new(0, 0), Point::new(7, 0))
                .stroke(Some(EpdColor::Black))
                .into_iter(),
        );

        let buffer = display.buffer();

        assert_eq!(buffer[0], EpdColor::Black);

        for &byte in buffer.iter().skip(8) {
            assert_eq!(byte, DEFAULT_BACKGROUND_COLOR);
        }
    }
}
