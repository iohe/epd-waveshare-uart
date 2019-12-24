use epd_waveshare_uart::{
    epd4in3::{Display4in3, EPD4in3},
    graphics::{Display, DisplayRotation},
    prelude::*,
};
use linux_embedded_hal::{
    Delay,
};

use std::thread;
use std::time::Duration;

use embedded_graphics::{prelude::*, fonts::*, coord::*, style::*, transform::*, primitives::*};

use rppal::uart::*;
use rppal::gpio::Gpio;


use std::path::Path;

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
    let mut serial = Uart::new( 115200, Parity::None, 8, 1)?;
    serial.set_hardware_flow_control(false);
    serial.set_software_flow_control(false);
    serial.set_rts(false);
    serial.set_write_mode(true);

    // Configure Digital I/O Pin to be used as wake-up and rst
    let gpio = Gpio::new()?;
    let mut wake = gpio.get(2)?.into_output();
    wake.set_low();
    
    let gpio = Gpio::new()?;
    let mut rst = gpio.get(4)?.into_output();

/*
    rst.set_low();
    thread::sleep(Duration::from_millis(500));
    rst.set_high();
    thread::sleep(Duration::from_millis(2500));
    rst.set_low();
    thread::sleep(Duration::from_millis(500));

  

    thread::sleep(Duration::from_millis(3000));


    wake.set_low();
    thread::sleep(Duration::from_millis(500));
    wake.set_high();
    thread::sleep(Duration::from_millis(500));
    wake.set_low();
    thread::sleep(Duration::from_millis(500));
*/
    /*
    println!("Handhsake");

    let handshake = [0xA5, 00 ,09 ,00, 0xCC, 0x33, 0xC3, 0x3C, 0xAC];
    serial.write(&handshake);

    thread::sleep(Duration::from_millis(500));

    
    let color = [0xA5, 00, 0x0B, 0x10, 0x00, 0x03, 0xCC, 0x33, 0xC3, 0x3C, 0xBD];
    serial.write(&color);
    println!("Color");

    let mut read_buffer = [0; 100];
    serial.read(&mut read_buffer);
    thread::sleep(Duration::from_millis(500));

    let direction = [0xa5, 0x00 , 0x0a , 0x0d , 0x01 , 0xcc , 0x33 , 0xc3 , 0x3c , 0xa3];
    serial.write(&direction);
    println!("Direction");
    let mut read_buffer = [0; 100];
    serial.read(&mut read_buffer);
    thread::sleep(Duration::from_millis(500));



    let rect = [0xA5, 0x00, 0x11, 0x24, 0x00, 0x0A, 0x00, 0x0A, 0x00, 0xFF, 0x00, 0xFF, 0xCC, 0x33, 0xC3, 0x3C, 0x90];
    serial.write(&rect);
    thread::sleep(Duration::from_millis(500));
    println!("Rect");

    let line = [0xa5, 0, 0x11, 0x22, 0, 0, 0, 0xa, 0, 0x14, 0, 0xa, 0xcc, 0x33, 0xc3, 0x3c, 0x82];
    serial.write(&line);
    thread::sleep(Duration::from_millis(500));
    println!("Line");

    let display_bytes = [0xa5, 0, 0x9, 0xa, 0xcc, 0x33, 0xc3, 0x3c, 0xa6];
    serial.write(&display_bytes);
    thread::sleep(Duration::from_millis(500));
    println!("Show Display");
    */
    
    let mut delay = Delay {};
    let mut display = Display4in3::default();
    let mut epd4in3 =
        EPD4in3::new(&mut serial, wake, rst, &mut delay).expect("eink initalize error");
    
    thread::sleep(Duration::from_millis(6000));


    /*
    display.draw(
                Font12x16::render_str("Awesoom tooolllll  and is never going to stop")
                 .stroke(Some(Color::Black))
                 .fill(Some(Color::White))
                 .translate(Coord::new(50, 200))
                 .into_iter(),
         );
      */
      
    
 /*     
    display.draw(
            Font12x16::render_str("Awesoom tooolllll  and is never going to stop")
             .stroke(Some(Color::Black))
             .fill(Some(Color::White))
             .translate(Coord::new(50, 100))
             .into_iter(),
     );
     


    display.draw(
        Font12x16::render_str("Test")
         .stroke(Some(Color::Black))
         .fill(Some(Color::White))
         .translate(Coord::new(200, 200))
         .into_iter(),
    );

    display.draw(
        Font12x16::render_str("La Multi Ani")
         .stroke(Some(Color::Black))
         .fill(Some(Color::White))
         .translate(Coord::new(300, 300))
         .into_iter(),
    );

    display.draw(
        Circle::new(Coord::new(64, 64), 64)
        .stroke(Some(Color::Black))
        .translate(Coord::new(300,300))
        .into_iter(),
    );

    
    display.draw(
        Font12x16::render_str("La Multi Ani")
         .stroke(Some(Color::Black))
         .fill(Some(Color::White))
         .translate(Coord::new(50, 100))
         .into_iter(),
    );
    */
    
    
    display.draw(Rectangle::new(Coord::new(100,200), Coord::new(408,400)).stroke(Some(Color::Black)).into_iter(),);

    epd4in3.update_frame(&mut serial, &display.buffer(),&mut delay).unwrap();    
    epd4in3.display_frame(&mut serial).expect("display frame new graphics");
    
    //display.draw(Rectangle::new(Coord::new(50,100), Coord::new(150,300)).stroke(Some(Color::Black)).into_iter(),);
    //epd4in3.update_frame(&mut serial, &display.buffer(),&mut delay).unwrap();    
    //epd4in3.display_frame(&mut serial).expect("display frame new graphics");
    

    thread::sleep(Duration::from_millis(5000));
    Ok(())
}
