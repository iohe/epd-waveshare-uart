use epd_waveshare_uart::{
    epd4in3::{Display4in3, EPD4in3},
    graphics::Display,
    prelude::*,
};
use linux_embedded_hal::Delay;

use std::thread;
use std::time::Duration;

use embedded_graphics::{prelude::*, primitives::*};

use rppal::gpio::Gpio;
use rppal::uart::*;

// activate uart, gpio in raspi-config
// needs to be run with sudo because of some sysfs_gpio permission problems and follow-up timing problems
// see https://github.com/rust-embedded/rust-sysfs-gpio/issues/5 and follow-up issues

fn main() {
    if let Err(e) = run() {
        eprintln!("Program exited early with error: {}", e);
    }
}

fn run() -> Result<()> {
    // Configure UART
    let mut serial = Uart::new(115_200, Parity::None, 8, 1)?;
    serial.set_hardware_flow_control(false).unwrap();
    serial.set_software_flow_control(false).unwrap();
    serial.set_rts(false).unwrap();
    serial.set_write_mode(true).unwrap();

    // Configure Digital I/O Pin to be used as wake-up and rst
    let gpio = Gpio::new()?;
    let mut wake = gpio.get(2)?.into_output();
    wake.set_low();

    let gpio = Gpio::new()?;
    let mut rst = gpio.get(4)?.into_output();
    rst.set_low();

    let mut delay = Delay {};
    let mut display = Display4in3::default();
    let mut epd4in3 =
        EPD4in3::new(&mut serial, wake, rst, &mut delay).expect("eink initalize error");

    thread::sleep(Duration::from_millis(3000));

    display.draw(
        Rectangle::new(Point::new(300, 200), Point::new(320, 230))
            .fill(Some(EpdColor::Gray))
            .stroke(Some(EpdColor::DarkGray))
            .into_iter(),
    );
    display.draw(
        Rectangle::new(Point::new(320, 200), Point::new(340, 230))
            .fill(Some(EpdColor::DarkGray))
            .stroke(Some(EpdColor::DarkGray))
            .into_iter(),
    );
    display.draw(
        Rectangle::new(Point::new(340, 200), Point::new(360, 230))
            .fill(Some(EpdColor::Black))
            .stroke(Some(EpdColor::DarkGray))
            .into_iter(),
    );
    display.draw(
        Circle::new(Point::new(400, 200), 100)
            .stroke(Some(EpdColor::Black))
            .into_iter(),
    );

    //Draw second eye
    display.draw(
        Rectangle::new(Point::new(300, 400), Point::new(320, 430))
            .fill(Some(EpdColor::Gray))
            .stroke(Some(EpdColor::DarkGray))
            .into_iter(),
    );
    display.draw(
        Rectangle::new(Point::new(320, 400), Point::new(340, 430))
            .fill(Some(EpdColor::DarkGray))
            .stroke(Some(EpdColor::DarkGray))
            .into_iter(),
    );
    display.draw(
        Rectangle::new(Point::new(340, 400), Point::new(360, 430))
            .fill(Some(EpdColor::Black))
            .stroke(Some(EpdColor::DarkGray))
            .into_iter(),
    );
    display.draw(
        Circle::new(Point::new(400, 400), 100)
            .stroke(Some(EpdColor::Black))
            .into_iter(),
    );

    epd4in3
        .update_frame(&mut serial, &display.buffer(), &mut delay)
        .unwrap();
    epd4in3
        .display_frame(&mut serial)
        .expect("display frame new graphics problem encountered");

    thread::sleep(Duration::from_millis(4000));
    Ok(())
}
