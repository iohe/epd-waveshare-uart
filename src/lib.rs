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
//! use epd_waveshare::{
//!     epd4in3::{EPD4in3, Display4in3},
//!     graphics::{Display, DisplayRotation},
//!     prelude::*,
//! };
//! use embedded_graphics::Drawing;
//!
//! // Setup EPD
//! let mut epd = EPD4in3::new(&mut uart, wake, rst, &mut delay).unwrap();
//!
//! // Use display graphics
//! let mut display = Display4in3::default();
//!
//! // Write some hello world in the screenbuffer
//! display.draw(
//!     Font6x8::render_str("Hello World!")
//!         .stroke(Some(Color::Black))
//!         .fill(Some(Color::White))
//!         .translate(Coord::new(5, 50))
//!         .into_iter(),
//! );
//!
//! // Display updated frame
//! epd.update_frame(&mut spi, &display.buffer()).unwrap();
//! epd.display_frame(&mut spi).expect("display frame new graphics");
//!
//! // Set the EPD to sleep
//! epd.sleep(&mut spi).expect("sleep");
//! ```
//!
//!
#![no_std]

#[cfg(feature = "graphics")]
pub mod graphics;

mod traits;

pub mod color;

/// Interface for the physical connection between display and the controlling device
mod interface;

#[cfg(feature = "epd4in3")]
pub mod epd4in3;

pub mod prelude {
    pub use crate::color::Color;
    pub use crate::traits::WaveshareDisplay;

    #[cfg(feature = "graphics")]
    pub use crate::graphics::{Display, DisplayRotation};
}
