use crate::traits::{Command, Error};
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

impl<E, F, SERIAL, WAKE, RST> DisplayInterface<SERIAL, WAKE, RST>
where
    SERIAL: Write<u8, Error = F> + Read<u8, Error = E>,
    WAKE: OutputPin,
    RST: OutputPin,
{
    pub fn new(wake: WAKE, rst: RST) -> Self {
        DisplayInterface {
            _serial: PhantomData::default(),
            wake,
            rst,
        }
    }

    /// Basic function for sending an array of u8-values of data over serial
    ///
    /// Enables direct interaction with the device with the help of [command()](EPD4in3::command())
    pub(crate) fn data(&mut self, serial: &mut SERIAL, data: &[u8]) -> Result<(), Error<E, F>> {
        // Transfer data (u8-array) over serial
        self.write(serial, data)
    }

    /// Basic function for reading an array of u8-values of data over serial
    ///
    /// Enables direct interaction with the device with the help of [command()](EPD4in3::command())
    pub(crate) fn read_serial(
        &mut self,
        serial: &mut SERIAL,
        data: &mut [u8],
    ) -> Result<(), Error<E, F>> {
        // Read data (u8-array) over serial
        self.read(serial, data)
    }

    // serial write helper/abstraction function
    pub(crate) fn read(&mut self, serial: &mut SERIAL, data: &mut [u8]) -> Result<(), Error<E, F>> {
        

        for i in 0..data.len() {
            match serial.read() {
                Ok(byte) => data[i] = byte,
                Err(e) => {},//Err(e),
            };
        }
        extern crate std;
        std::println!(">>{:x?}", data);

        Ok(())
    }

    // serial write helper/abstraction function
    fn write(&mut self, serial: &mut SERIAL, data: &[u8]) -> Result<(), Error<E, F>> {
        extern crate std;
        std::println!("{:x?}", data);
        serial.bwrite_all(data)?;

        Ok(())
    }

    /// Resets the device.
    ///
    /// Often used to awake the module from deep sleep. See [EPD4in3::sleep()](EPD4in3::sleep())
    ///
    /// TODO: Takes at least 400ms of delay alone, can it be shortened?
    pub(crate) fn reset<DELAY: DelayMs<u16>>(&mut self, delay: &mut DELAY) {
        self.rst.set_low();
        //TODO: why 200ms? (besides being in the arduino version)
        delay.delay_ms(255);
        self.rst.set_high();
        //TODO: same as 3 lines above
        delay.delay_ms(3000);
        self.rst.set_low();
        delay.delay_ms(255);
        
    }

    /// Wakes the device.
    ///
    /// Often used to awake the module from deep sleep. See [EPD4in3::sleep()](EPD4in3::sleep())
    ///
    /// TODO: Takes at least 400ms of delay alone, can it be shortened?
    pub(crate) fn wake<DELAY: DelayMs<u16>>(&mut self, delay: &mut DELAY) {
        self.wake.set_low();
        //TODO: why 200ms? (besides being in the arduino version)
        delay.delay_ms(255);
        self.wake.set_high();
        //TODO: same as 3 lines above
        delay.delay_ms(255);
        self.wake.set_low();
        delay.delay_ms(255);
    }
}
