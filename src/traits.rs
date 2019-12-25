use crate::color::EpdColor;
use core::marker::Sized;
use embedded_hal::{
    blocking::{delay::*, serial::Write},
    digital::v2::*,
    serial::Read,
};

/// All commands need to have this trait which gives the address of the command
/// which needs to be send via Serial
pub(crate) trait Command {
    fn address(self) -> u8;
}

/// Errors
#[derive(Debug)]
pub enum Error<E, F, G> {
    /// Serial read bus error
    SerialR(E),
    /// Serial write error
    SerialW(F),
    /// GPIO Error
    GpioE(G),
    /// Timeout
    Timeout,
}

pub(crate) trait InternalWiAdditions<E, F, G, SERIAL, WAKE, RST>
where
    SERIAL: Write<u8, Error = F> + Read<u8, Error = E>,
    WAKE: OutputPin<Error = G>,
    RST: OutputPin<Error = G>,
{
    /// This initialises the EPD and powers it up
    ///
    /// This function is already called from
    ///  - [new()](WaveshareInterface::new())
    ///  - [`wake_up`]
    ///
    ///
    /// This function calls [reset()](WaveshareInterface::reset()),
    /// so you don't need to call reset your self when trying to wake your device up
    /// after setting it to sleep.
    fn init<DELAY: DelayMs<u16>>(
        &mut self,
        serial: &mut SERIAL,
        delay: &mut DELAY,
    ) -> Result<(), Error<E, F, G>>;

    fn wake<DELAY: DelayMs<u16>>(
        &mut self,
        serial: &mut SERIAL,
        delay: &mut DELAY,
    ) -> Result<(), Error<E, F, G>>;
}

/// All the functions to interact with the EPDs
///
/// This trait includes all public functions to use the EPDS
pub trait WaveshareDisplay<E, F, G, SERIAL, WAKE, RST>
where
    SERIAL: Write<u8, Error = F> + Read<u8, Error = E>,
    WAKE: OutputPin<Error = G>,
    RST: OutputPin<Error = G>,
{
    /// Creates a new driver from a Serial peripheral, Wake Pin, Reset Pin
    ///
    /// This already initialises the device. That means [init()](WaveshareInterface::init()) isn't needed directly afterwards
    fn new<DELAY: DelayMs<u16>>(
        serial: &mut SERIAL,
        wake: WAKE,
        rst: RST,
        delay: &mut DELAY,
    ) -> Result<Self, Error<E, F, G>>
    where
        Self: Sized;

    /// Let the device enter deep-sleep mode to save power.
    ///
    /// The deep sleep mode returns to standby with a hardware reset.
    /// But you can also use [wake_up()](WaveshareInterface::wake_up()) to awaken.
    /// But as you need to power it up once more anyway you can also just directly use [new()](WaveshareInterface::new()) for resetting
    /// and initialising which already contains the reset
    fn sleep(&mut self, serial: &mut SERIAL) -> Result<(), Error<E, F, G>>;

    /// Wakes the device up from sleep
    fn wake_up<DELAY: DelayMs<u16>>(
        &mut self,
        serial: &mut SERIAL,
        delay: &mut DELAY,
    ) -> Result<(), Error<E, F, G>>;

    /// Sets the backgroundcolor for various commands like [clear_frame()](WaveshareInterface::clear_frame())
    fn set_background_color(&mut self, color: EpdColor);

    /// Sets the foregroundcolor for various commands like [clear_frame()](WaveshareInterface::clear_frame())
    fn set_foreground_color(&mut self, color: EpdColor);

    /// Get the width of the display
    fn width(&self) -> u32;

    /// Get the height of the display
    fn height(&self) -> u32;

    /// Transmit a full frame to the SRAM of the EPD
    fn update_frame<DELAY: DelayMs<u16>>(
        &mut self,
        serial: &mut SERIAL,
        buffer: &[EpdColor],
        delay: &mut DELAY,
    ) -> Result<(), Error<E, F, G>>;

    /// Displays the frame data from SRAM
    fn display_frame(&mut self, serial: &mut SERIAL) -> Result<(), Error<E, F, G>>;

    /// Clears the frame buffer on the EPD with the declared background color
    ///
    /// The background color can be changed with [`set_background_color`]
    fn clear_frame(&mut self, serial: &mut SERIAL) -> Result<(), Error<E, F, G>>;
}
