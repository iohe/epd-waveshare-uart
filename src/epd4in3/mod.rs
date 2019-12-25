//! A simple Driver for the Waveshare 4.3" E-Ink Display via SERIAL
//!
//!
//! Build with the help of documentation/code from [Waveshare](https://www.waveshare.com/wiki/4.2inch_e-Paper_Module),
//! [Ben Krasnows partial Refresh tips](https://benkrasnow.blogspot.de/2017/10/fast-partial-refresh-on-42-e-paper.html) and
//! the driver documents in the `pdfs`-folder as orientation.
//!
//! This driver was built using [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://docs.rs/embedded-hal/~0.1
//!
//! # Requirements
//!
//! ### SERIAL
//!
//! - 8 bits per word, MSB first
//! - 1 bit stop
//! - Max. Speed tested was 8Mhz but more should be possible
//!
//! ### Other....
//!
//! - Buffersize: Wherever a buffer is used it always needs to be of the size: `width / 8 * length`,
//!   where width and length being either the full e-ink size, since it does not support partial update window size
//!
//! # Examples
//!
//! ```ignore
//! let mut epd4in3 = EPD4in3::new(serial, wake, rst, delay).unwrap();
//!
//! let mut buffer =  [0u8, epd4in3.get_width() * epd4in3.get_height()];
//!
//! // draw something into the buffer
//!
//! epd4in3.update_frame(&mut serial, &display.buffer(), &mut delay)?;
//!
//! epd4in3.display_frame(&mut serial)?;
//! 
//! // wait and look at the image
//!
//! epd4in3.clear_frame(None);
//!
//! epd4in3.sleep();
//! ```
//!
//!

//use crate::graphics::DisplayRotation;
use embedded_hal::{
    blocking::{delay::*, serial::Write},
    digital::v2::*,
    serial::Read,
};

use crate::color::EpdColor;
use crate::interface::DisplayInterface;
use crate::traits::{Error, InternalWiAdditions, WaveshareDisplay};

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 600;
pub const DEFAULT_BACKGROUND_COLOR: EpdColor = EpdColor::White;
pub const DEFAULT_FOREGROUND_COLOR: EpdColor = EpdColor::Black;

pub mod command;

#[cfg(feature = "graphics")]
mod graphics;
#[cfg(feature = "graphics")]
pub use self::graphics::Display4in3;

/// EPD4in3 driver
///
pub struct EPD4in3<SERIAL, WAKE, RST> {
    /// Connection Interface
    interface: DisplayInterface<SERIAL, WAKE, RST>,
    /// Background Color
    bg_color: EpdColor,
    /// Foreground Color
    fg_color: EpdColor,
}

impl<E, F, G, SERIAL, WAKE, RST> InternalWiAdditions<E, F, G, SERIAL, WAKE, RST>
    for EPD4in3<SERIAL, WAKE, RST>
where
    SERIAL: Write<u8, Error = F> + Read<u8, Error = E>,
    WAKE: OutputPin<Error = G>,
    RST: OutputPin<Error = G>,
{
    fn init<DELAY: DelayMs<u16>>(
        &mut self,
        _serial: &mut SERIAL,
        delay: &mut DELAY,
    ) -> Result<(), Error<E, F, G>> {
        // reset the device
        self.interface.reset(delay)
    }

    fn wake<DELAY: DelayMs<u16>>(
        &mut self,
        _serial: &mut SERIAL,
        delay: &mut DELAY,
    ) -> Result<(), Error<E, F, G>> {
        // wakes the device
        self.interface.wake(delay)
    }
}

impl<E, F, G, SERIAL, WAKE, RST> WaveshareDisplay<E, F, G, SERIAL, WAKE, RST>
    for EPD4in3<SERIAL, WAKE, RST>
where
    SERIAL: Write<u8, Error = F> + Read<u8, Error = E>,
    WAKE: OutputPin<Error = G>,
    RST: OutputPin<Error = G>,
{
    /// Creates a new driver from a SERIAL peripheral, WAKE Pin, RST Pin
    ///
    /// This already initialises the device. That means [init()](init()) isn't needed directly afterwards
    ///
    /// # Example
    ///
    /// ```ignore
    /// //buffer = some image data;
    ///
    /// let mut epd4in3 = EPD4in3::new(serial, wake, rst, delay);
    ///
    /// epd4in3.display_and_transfer_frame(buffer, None);
    ///
    /// epd4in3.sleep();
    /// ```
    fn new<DELAY: DelayMs<u16>>(
        serial: &mut SERIAL,
        wake: WAKE,
        rst: RST,
        delay: &mut DELAY,
    ) -> Result<Self, Error<E, F, G>> {
        let interface = DisplayInterface::new(wake, rst);
        let bg_color = DEFAULT_BACKGROUND_COLOR;
        let fg_color = DEFAULT_FOREGROUND_COLOR;
        let mut epd = EPD4in3 {
            interface,
            bg_color,
            fg_color,
        };

        epd.init(serial, delay)?;

        Ok(epd)
    }

    fn wake_up<DELAY: DelayMs<u16>>(
        &mut self,
        serial: &mut SERIAL,
        delay: &mut DELAY,
    ) -> Result<(), Error<E, F, G>> {
        self.wake(serial, delay)
    }

    fn sleep(&mut self, serial: &mut SERIAL) -> Result<(), Error<E, F, G>> {
        let cmd_sleep = command::sleep().unwrap();
        self.interface.data(serial, cmd_sleep.get_bytes())?;
        Ok(())
    }

    fn update_frame<DELAY: DelayMs<u16>>(
        &mut self,
        serial: &mut SERIAL,
        buffer: &[EpdColor],
        _delay: &mut DELAY,
    ) -> Result<(), Error<E, F, G>> {
        for (index, &color) in buffer.iter().enumerate() {
            let mut retries = 0;
            let mut response_ok = false;
            while (retries < 10) && !response_ok {
                let mut read_bytes = 0;
                //Skip background pixels
                if self.bg_color == color {
                    break;
                }

                if self.fg_color != color {
                    self.set_foreground_color(color);
                    let cmd_color = command::set_color(color, self.bg_color).unwrap();
                    self.interface.data(serial, cmd_color.get_bytes())?;
                    read_bytes += 2;
                }

                let (x, y) = (index as u32 % self.width(), index as u32 / self.width());
                let cmd_point = command::point(x as u16, y as u16).unwrap();
                self.interface.data(serial, cmd_point.get_bytes())?;
                //delay.delay_ms(20);
                read_bytes += 2;

                let mut data = [0u8; 4];
                if read_bytes > 0 {
                    response_ok = true;
                    self.interface
                        .read_serial(serial, &mut data[0..read_bytes])?;
                    for &byte in data[0..read_bytes].iter() {
                        if byte == 0x00 {
                            response_ok = false;
                            retries+=1;
                            //extern crate std;
                            //std::println!("{:x?}", data);
                        }
                    }
                } else {
                    retries = 10;
                }
            } //end while
        } //end for

        Ok(())
    }

    fn display_frame(&mut self, serial: &mut SERIAL) -> Result<(), Error<E, F, G>> {
        let cmd = command::refresh().unwrap();
        let bytes = cmd.get_bytes();
        self.interface.data(serial, bytes)
    }

    fn clear_frame(&mut self, serial: &mut SERIAL) -> Result<(), Error<E, F, G>> {
        let cmd = command::clear().unwrap();
        let bytes = cmd.get_bytes();
        self.interface.data(serial, bytes)
    }

    fn set_background_color(&mut self, color: EpdColor) {
        self.bg_color = color;
    }

    fn set_foreground_color(&mut self, color: EpdColor) {
        self.fg_color = color;
    }

    fn width(&self) -> u32 {
        WIDTH
    }

    fn height(&self) -> u32 {
        HEIGHT
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn epd_size() {
        assert_eq!(WIDTH, 800);
        assert_eq!(HEIGHT, 600);
        assert_eq!(DEFAULT_BACKGROUND_COLOR, EpdColor::White);
    }
}
