use crate::traits::Error;
use core::marker::PhantomData;
use embedded_hal::{
    blocking::{delay::*, serial::Write},
    digital::v2::*,
    serial::Read,
};

/// The Connection Interface of 4.3 Waveshare EPD-Devices
///
pub(crate) struct DisplayInterface<SERIAL, WAKE, RST> {
    /// SERIAL
    _serial: PhantomData<SERIAL>,
    /// Pin for Wake-up
    wake: WAKE,
    /// Pin for Reseting
    rst: RST,
}

impl<E, F, G, SERIAL, WAKE, RST> DisplayInterface<SERIAL, WAKE, RST>
where
    SERIAL: Write<u8, Error = F> + Read<u8, Error = E>,
    WAKE: OutputPin<Error = G>,
    RST: OutputPin<Error = G>,
{
    //type Error = Error<E, F, G>;

    pub fn new(wake: WAKE, rst: RST) -> Self {
        DisplayInterface {
            _serial: PhantomData::default(),
            wake,
            rst,
        }
    }

    /// Basic function for sending an array of u8-values of data over serial
    ///
    /// Enables direct interaction with the device
    pub(crate) fn data(&mut self, serial: &mut SERIAL, data: &[u8]) -> Result<(), Error<E, F, G>> {
        // Transfer data (u8-array) over serial
        self.write(serial, data)
    }

    /// Basic function for reading an array of u8-values of data over serial
    ///
    /// Enables direct interaction with the device
    pub(crate) fn read_serial(
        &mut self,
        serial: &mut SERIAL,
        data: &mut [u8],
    ) -> Result<(), Error<E, F, G>> {
        // Read data (u8-array) over serial
        self.read(serial, data)
    }

    // serial write helper/abstraction function
    pub(crate) fn read(
        &mut self,
        serial: &mut SERIAL,
        data: &mut [u8],
    ) -> Result<(), Error<E, F, G>> {
        for item in data.iter_mut() {
            match serial.read() {
                Ok(byte) => *item = byte,
                Err(_e) => {} //Err(e),
            };
        }
        //extern crate std;
        //std::println!(">>{:x?}", data);

        Ok(())
    }

    // serial write helper/abstraction function
    fn write(&mut self, serial: &mut SERIAL, data: &[u8]) -> Result<(), Error<E, F, G>> {
        //extern crate std;
        //std::println!("{:x?}", data);
        serial.bwrite_all(data).map_err(Error::SerialW)
    }

    /// Resets the device.
    ///
    /// Often used to awake the module from deep sleep. See [EPD4in3::sleep()](EPD4in3::sleep())
    ///
    /// TODO: Takes at least 400ms of delay alone, can it be shortened?
    pub(crate) fn reset<DELAY: DelayMs<u16>>(
        &mut self,
        delay: &mut DELAY,
    ) -> Result<(), Error<E, F, G>> {
        self.rst.set_low().map_err(Error::GpioE)?;
        //TODO: why 200ms? (besides being in the arduino version)
        delay.delay_ms(255);
        self.rst.set_high().map_err(Error::GpioE)?;
        //TODO: same as 3 lines above
        delay.delay_ms(3000);
        self.rst.set_low().map_err(Error::GpioE)?;
        delay.delay_ms(255);
        Ok(())
    }

    /// Wakes the device.
    ///
    /// Often used to awake the module from deep sleep. See [EPD4in3::sleep()](EPD4in3::sleep())
    ///
    /// TODO: Takes at least 400ms of delay alone, can it be shortened?
    pub(crate) fn wake<DELAY: DelayMs<u16>>(
        &mut self,
        delay: &mut DELAY,
    ) -> Result<(), Error<E, F, G>> {
        self.wake.set_low().map_err(Error::GpioE)?;
        //TODO: why 200ms? (besides being in the arduino version)
        delay.delay_ms(255);
        self.wake.set_high().map_err(Error::GpioE)?;
        //TODO: same as 3 lines above
        delay.delay_ms(255);
        self.wake.set_low().map_err(Error::GpioE)?;
        delay.delay_ms(255);
        Ok(())
    }
}
