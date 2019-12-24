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
//! let mut buffer =  [0u8, epd4in3.get_width() / 8 * epd4in3.get_height()];
//!
//! // draw something into the buffer
//!
//! epd4in3.display_and_transfer_buffer(buffer, None);
//!
//! // wait and look at the image
//!
//! epd4in3.clear_frame(None);
//!
//! epd4in3.sleep();
//! ```
//!
//!

use crate::graphics::DisplayRotation;
use embedded_hal::{
    blocking::{delay::*, serial::Write},
    digital::v2::*,
    serial::Read,
};

use crate::interface::DisplayInterface;
use crate::traits::{Error, InternalWiAdditions, WaveshareDisplay};

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 600;
pub const DEFAULT_BACKGROUND_COLOR: Color = Color::White;

use crate::color::Color;

pub(crate) mod command;
use self::command::Command;

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
    color: Color,
}

impl<E, F, SERIAL, WAKE, RST> InternalWiAdditions<E, F, SERIAL, WAKE, RST>
    for EPD4in3<SERIAL, WAKE, RST>
where
    SERIAL: Write<u8, Error = F> + Read<u8, Error = E>,
    WAKE: OutputPin,
    RST: OutputPin,
{
    fn init<DELAY: DelayMs<u16>>(
        &mut self,
        _serial: &mut SERIAL,
        delay: &mut DELAY,
    ) -> Result<(), Error<E, F>> {
        // reset the device
        self.interface.reset(delay);

        Ok(())
    }

    fn wake<DELAY: DelayMs<u16>>(
        &mut self,
        _serial: &mut SERIAL,
        delay: &mut DELAY,
    ) -> Result<(), Error<E, F>> {
        // wakes the device
        self.interface.wake(delay);

        Ok(())
    }
}

impl<E, F, SERIAL, WAKE, RST> WaveshareDisplay<E, F, SERIAL, WAKE, RST>
    for EPD4in3<SERIAL, WAKE, RST>
where
    SERIAL: Write<u8, Error = F> + Read<u8, Error = E>,
    WAKE: OutputPin,
    RST: OutputPin,
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
    ) -> Result<Self, Error<E, F>> {
        let interface = DisplayInterface::new(wake, rst);
        let color = DEFAULT_BACKGROUND_COLOR;

        let mut epd = EPD4in3 { interface, color };

        epd.init(serial, delay)?;

        Ok(epd)
    }

    fn wake_up<DELAY: DelayMs<u16>>(
        &mut self,
        serial: &mut SERIAL,
        delay: &mut DELAY,
    ) -> Result<(), Error<E, F>> {
        self.wake(serial, delay)
    }

    fn sleep(&mut self, serial: &mut SERIAL) -> Result<(), Error<E, F>> {
        /*        self.interface
                    .cmd_with_data(serial, Command::VCOM_AND_DATA_INTERVAL_SETTING, &[0x17])?; //border floating
                self.command(serial, Command::VCM_DC_SETTING)?; // VCOM to 0V
                self.command(serial, Command::PANEL_SETTING)?;

                self.command(serial, Command::POWER_SETTING)?; //VG&VS to 0V fast
                for _ in 0..4 {
                    self.send_data(serial, &[0x00])?;
                }

                self.command(serial, Command::POWER_OFF)?;
                //self.wait_until_idle();
                self.interface
                    .cmd_with_data(serial, Command::DEEP_SLEEP, &[0xA5])?;
        */
        //self.wait_until_idle();
        Ok(())
    }

    fn update_frame<DELAY: DelayMs<u16>>(&mut self, serial: &mut SERIAL, buffer: &[u8], delay: &mut DELAY) -> Result<(), Error<E, F>> {
    //fn update_frame(&mut self, serial: &mut SERIAL, buffer: &[u8]) -> Result<(), Error<E, F>> {
        //let color_value = self.color.get_byte_value();

        for (index, &byte) in buffer.iter().enumerate() {
            let mut retries = 0;
            let mut response_ok = false;
            while (retries < 10) && (response_ok == false) {
                let mut read_bytes = 0;
                for i in 0u8..7u8 {
                    //if self.color.get_bit_value()  == ((byte >> i) & 1)
                    if 1 == ((byte >> i) & 1) {
                        continue;
                    }

                    let (x, y) =
                        self.recover_position(index as u32, 1 << i, DisplayRotation::Rotate0);
                    let cmd_point = command::point(x as u16, y as u16).unwrap();
                    self.interface.data(serial, cmd_point.get_bytes())?;
                    delay.delay_ms(20);              
                    read_bytes += 2;
                }
                let mut data = [0u8; 16];
                if read_bytes > 0 {
                    response_ok = true;
                    self.interface
                        .read_serial(serial, &mut data[0..read_bytes])?;
                    for &byte in data[0..read_bytes].iter() {
                        if byte == 0x00 {
                            response_ok = false;
                        }
                    }
                } else {
                    retries = 10;
                }
            }
        } //end for

        Ok(())
    }

    fn display_frame(&mut self, serial: &mut SERIAL) -> Result<(), Error<E, F>> {
        let cmd = command::refresh().unwrap();
        let bytes = cmd.get_bytes();
        self.interface.data(serial, bytes)?;

        Ok(())
    }

    fn clear_frame(&mut self, serial: &mut SERIAL) -> Result<(), Error<E, F>> {
        let cmd = command::clear().unwrap();
        let bytes = cmd.get_bytes();
        self.interface.data(serial, bytes)?;

        extern crate std;
        std::println!("{:x?}", bytes);

        Ok(())
    }

    fn set_background_color(&mut self, color: Color) {
        self.color = color;
    }

    fn width(&self) -> u32 {
        WIDTH
    }

    fn height(&self) -> u32 {
        HEIGHT
    }
}

impl<E, F, SERIAL, WAKE, RST> EPD4in3<SERIAL, WAKE, RST>
where
    SERIAL: Write<u8, Error = F> + Read<u8, Error = E>,
    WAKE: OutputPin,
    RST: OutputPin,
{
    fn send_data(&mut self, serial: &mut SERIAL, data: &[u8]) -> Result<(), Error<E, F>> {
        self.interface.data(serial, data)
    }


    #[rustfmt::skip]
    //returns index position in the u8-slice and the bit-position inside that u8
    fn recover_position(&self, index:u32, bit_pos: u8, rotation: DisplayRotation) -> (u32, u32) {

        let modulo = match bit_pos {
            1 => 7,
            2 => 6,
            4 => 5,
            8 => 4,
            16 => 3,
            32 => 2,
            64 => 1,
            128 => 0,
            e => panic!{"Unexpected bit position {}",e},
        };
        match rotation {
            DisplayRotation::Rotate0 => (
                ((index*8) % self.width())+modulo as u32,
                (index*8) / self.width(),
                
            ),

            _ => ((500,500)),
            /*
            DisplayRotation::Rotate90 => (
                (self.width() - 1 - y) / 8 + (self.width() / 8) * x,
                0x01 << (y % 8) ,
            ),
            DisplayRotation::Rotate180 => (
                ((self.width() / 8) * self.height() - 1) - (x / 8 + (self.width() / 8) * y),
                0x01 << (x % 8) ,
            ),
            DisplayRotation::Rotate270 => (
                y / 8 + (height - 1 - x) * (width / 8),
                0x80 >> (y % 8),
            ),
            */
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn epd_size() {
        assert_eq!(WIDTH, 800);
        assert_eq!(HEIGHT, 600);
        assert_eq!(DEFAULT_BACKGROUND_COLOR, Color::White);
    }
}
