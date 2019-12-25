//! A simple Driver for the Waveshare E-Ink Displays via UART
//!
//! This driver was built using [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://docs.rs/embedded-hal/~0.1
//!
//! # Requirements
//!
//! ### UART
//!
//! - 8 bits per word, MSB first
//! - Max. Speed tested by myself was 8Mhz but more should be possible (Ben Krasnow used 18Mhz with his implemenation)
//!
//! ### Other....
//!
//! - Buffersize: Wherever a buffer is used it always needs to be of the size: `width / 8 * length`,
//!   where width and length being either the full e-ink size or the partial update window size
//!
//! # Examples
//!
//! ```rust,ignore
//! use epd_waveshare_uart::{
//!     epd4in3::{EPD4in3, Display4in3},
//!     graphics::{Display, DisplayRotation},
//!     prelude::*,
//! };
//! use embedded_graphics::Drawing;
//!
//! // Setup EPD
//! let mut epd = EPD4in3::new(&mut serial, wake, rst, &mut delay).unwrap();
//!
//! // Use display graphics
//! let mut display = Display4in3::default();
//!
//! // Write some hello world in the screenbuffer
//! display.draw(
//!     Font6x8::render_str("Hello World!")
//!         .stroke(Some(EpdColor::Black))
//!         .fill(Some(EpdColor::White))
//!         .translate(Point::new(5, 50))
//!         .into_iter(),
//! );
//!
//! // Display updated frame
//! epd.update_frame(&mut serial, &display.buffer()).unwrap();
//! epd.display_frame(&mut serial).expect("display frame new graphics");
//!
//! // Set the EPD to sleep
//! epd.sleep(&mut serial).expect("sleep");
//! ```
//!
//!
#![no_std]

#[cfg(feature = "graphics")]
pub mod graphics;

pub mod color;
/// Interface for the physical connection between display and the controlling device
mod interface;
mod traits;

#[cfg(feature = "epd4in3")]
pub mod epd4in3;
#[cfg(feature = "epd4in3")]
pub use crate::epd4in3::command;

pub mod prelude {
    pub use crate::color::EpdColor;
    pub use crate::traits::WaveshareDisplay;

    #[cfg(feature = "graphics")]
    pub use crate::graphics::{Display, DisplayRotation};
}
