[![Build Status](https://travis-ci.com/iohe/epd-waveshare-uart.svg?branch=master)](https://travis-ci.com/iohe/epd-waveshare-uart)

This library contains a driver for E-Paper Modules from Waveshare that use UART protocol. So far only 4in3 (800x600) is supported.

A 2018-edition compatible version (Rust 1.31+) is needed.

Other similiar libraries with support for much more displays are [epd-waveshare](https://github.com/Caemor/epd-waveshare),  [u8g2](https://github.com/olikraus/u8g2) and [GxEPD](https://github.com/ZinggJM/GxEPD) for arduino.

## Examples

There are multiple examples in the examples folder. For more infos about the examples see the seperate Readme [there](/examples/Readme.md). These examples are all rust projects of their own, so you need to go inside the project to execute it (cargo run --example doesn't work).

```Rust
// Setup the epd
let mut epd = EPD4in3::new(&mut serial, wake, rst, &mut delay)?;

// Setup the graphics
let mut buffer = Buffer4in3::default();
let mut display = Display::new(epd.width(), epd.height(), &mut buffer.buffer);

// Draw some text
display.draw(
    Font12x16::render_str("Hello Rust!")
        .stroke(Some(EpdColor::Black))
        .fill(Some(EpdColor::White))
        .translate(Point::new(5, 50))
        .into_iter(),
);

// Transfer the frame data to the epd
epd.update_frame(&mut serial, &display.buffer(), delay)?;

// Display the frame on the epd
epd.display_frame(&mut serial)?;
```

## (Supported) Devices

| Device (with Link) | Colors | Flexible Display | Partial Refresh | Supported | Tested |
| :---: | --- | :---: | :---: | :---: | :---: |
| [4.3 Inch B/W ](https://www.waveshare.com/4.3inch-e-paper.htm) | Black, White | ✕ | ✕ | ✔ | ✔ |

### Interface

| Interface | Description |
| :---: |  :--- |
| VCC 	|   5.0V |
| GND   | 	GND |
| DOUT  | 	Serial data out |
| DIN   | 	Serial data in |
| WAKE_UP | 	External wake up |
| RST   | 	External reset pin (Low for reset) |

