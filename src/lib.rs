#![no_std]

//! # is31fl3731 driver
//!
//! [![Crates.io](https://img.shields.io/crates/v/is31fl3731)](https://crates.io/crates/is31fl3731)
//! [![Crates.io](https://img.shields.io/crates/d/is31fl3731)](https://crates.io/crates/is31fl3731)
//! [![docs.rs](https://img.shields.io/docsrs/is31fl3731)](https://docs.rs/is31fl3731/latest/is31fl3731/)
//!
//! [![lint](https://github.com/gleich/is31fl3731/actions/workflows/lint.yml/badge.svg)](https://github.com/gleich/is31fl3731/actions/workflows/lint.yml)
//! [![build](https://github.com/gleich/is31fl3731/actions/workflows/build.yml/badge.svg)](https://github.com/gleich/is31fl3731/actions/workflows/build.yml)
//!
//! Driver for [Lumissil Microsystem's IS31FL3731 integrated circuit](https://www.lumissil.com/assets/pdf/core/IS31FL3731_DS.pdf). Some of the major features of this library are:
//!
//! 1. Use of embedded HAL traits (works with any embedded device that supports the required traits). This means that this driver is platform agnostic.
//! 2. Library features (only turn on what devices you need to save compiled binary space).
//! 3. [Examples](./examples) on how to use this driver. Right now there is only an example on how to use this crate with a raspberry pi. If you're looking for an embedded example check out my [random_matrix github repo](https://github.com/gleich/random_matrix) which uses this driver on the raspberry pi pico (rp2040 microcontroller).
//!
//! ## Install
//!
//! To install this driver in your project add the following line to your `Cargo.toml`'s `dependencies` table:
//!
//! ```toml
//! is31fl3731 = "1.0.0"
//! ```
//!
//! By default this version will only contain the core driver. To use a preconfigured device, such as the [Adafruit CharliePlex LED Matrix Bonnet](https://www.adafruit.com/product/3467), you would need to change this line to include that device:
//!
//! ```toml
//! is31fl3732 = { version = "1.0.0", features = ["charlie_bonnet"] }
//! ```
//!
//! ## Functionality & Plans
//!
//! Currently this library only supports some basic functions of the matrix (e.g. setup, fill, pixels). A few other features need to be implemented:
//!
//! - [ ] autoplay
//! - [ ] fade
//! - [ ] audio_play
//! - [ ] blink
//!
//! ## Inspiration
//!
//! This driver is a port of [adafruit's driver for the is31fl3731](https://github.com/adafruit/Adafruit_CircuitPython_IS31FL3731) in the rust programming language.

/// Preconfigured devices
pub mod devices;

use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::blocking::i2c::Write;

/// A struct to integrate with a new IS31FL3731 powered device.
pub struct IS31FL3731<I2C> {
    /// The i2c bus that is used to interact with the device. See implementation below for the
    /// trait methods required.
    pub i2c: I2C,
    /// The 7-bit i2c slave address of the device. By default on most devices this is `0x74`.
    pub address: u8,
    /// Width of the LED matrix
    pub width: u8,
    /// Height of the LED matrix
    pub height: u8,
    /// Method to convert an x,y coordinate pair to a binary address that can be accessed using the
    /// bus
    pub calc_pixel: fn(x: u8, y: u8) -> u8,
    frame: u8,
}

impl<I2C, I2cError> IS31FL3731<I2C>
where
    I2C: Write<Error = I2cError>,
{
    /// Fill the display with a single brightness. The brightness should range from 0 to 255. The reason that blink is an optional is
    /// because you can either set blink to true, set blink to false, or not set blink at all. The
    /// frame is the frame in which the fill should be applied to. Please consult the "General
    /// Description" section on the first page of the [data sheet](https://www.lumissil.com/assets/pdf/core/IS31FL3731_DS.pdf)
    /// for more information on frames.
    pub fn fill(&mut self, brightness: u8, blink: Option<bool>, frame: u8) -> Result<(), I2cError> {
        self.bank(frame)?;
        let mut payload = [brightness; 25];
        for row in 0..6 {
            payload[0] = addresses::COLOR_OFFSET + row * 24;
            self.i2c.write(self.address, &payload)?;
        }
        if blink.is_some() {
            let data = if blink.unwrap() { 1 } else { 0 } * 0xFF;
            for col in 0..18 {
                self.write_register(frame, addresses::BLINK_OFFSET + col, data)?;
            }
        }
        Ok(())
    }

    /// Setup the display. Should be called before interacting with the device to ensure proper
    /// functionality. Delay is something that your device's HAL should provide which allows for
    /// the process to sleep for a certain amount of time (in this case 10 MS to perform a reset).
    ///
    /// When you run this function the following steps will occur:
    /// 1. The chip will be told that it's being "reset".
    /// 2. All frames will be cleared.
    /// 3. Audio syncing will be turned off.
    /// 4. The chip will be told that it's being turned back on.
    pub fn setup<DEL: DelayMs<u8>>(&mut self, delay: &mut DEL) -> Result<(), Error<I2cError>> {
        self.sleep(true)?;
        delay.delay_ms(10);
        self.mode(addresses::PICTURE_MODE)?;
        self.frame(0)?;
        for frame in 0..8 {
            self.fill(0, Some(false), frame)?;
            for col in 0..18 {
                self.write_register(frame, addresses::ENABLE_OFFSET + col, 0xFF)?;
            }
        }
        self.audio_sync(false)?;
        self.sleep(false)?;
        Ok(())
    }

    /// Set the brightness at a specific x,y coordinate. Just like the [fill method](Self::fill)
    /// the brightness should range from 0 to 255. If the coordinate is out of range then the
    /// function will return an error of [InvalidLocation](Error::InvalidLocation).
    pub fn pixel(&mut self, x: u8, y: u8, brightness: u8) -> Result<(), Error<I2cError>> {
        if x > self.width {
            return Err(Error::InvalidLocation(x));
        }
        if y > self.height {
            return Err(Error::InvalidLocation(y));
        }
        let pixel = (self.calc_pixel)(x, y);
        self.write_register(self.frame, addresses::COLOR_OFFSET + pixel, brightness)?;
        Ok(())
    }

    /// Change the slave address to a new 7-bit address. Should be configured before calling
    /// [setup](Self::setup) method.
    pub fn set_address(&mut self, address: u8) {
        self.address = address;
    }

    /// Set frame ranging from 0 to 8. Please consult the "General Description" section on the
    /// first page of the [data sheet](https://www.lumissil.com/assets/pdf/core/IS31FL3731_DS.pdf)
    /// for more information on frames.
    pub fn frame(&mut self, frame: u8) -> Result<(), Error<I2cError>> {
        if frame > 8 {
            return Err(Error::InvalidLocation(frame));
        }
        self.frame = frame;
        self.write_register(addresses::CONFIG_BANK, addresses::FRAME, frame)?;
        Ok(())
    }

    /// Send a reset message to the slave device. Delay is something that your device's HAL should
    /// provide which allows for the process to sleep for a certain amount of time (in this case 10
    /// MS to perform a reset).
    pub fn reset<DEL: DelayMs<u8>>(&mut self, delay: &mut DEL) -> Result<(), I2cError> {
        self.sleep(true)?;
        delay.delay_ms(10);
        self.sleep(false)?;
        Ok(())
    }

    /// Set the device mode. Please consult page 17 and 18 of the [data sheet](https://www.lumissil.com/assets/pdf/core/IS31FL3731_DS.pdf)
    /// to learn mode about the different modes.
    pub fn mode(&mut self, mode: u8) -> Result<(), I2cError> {
        self.write_register(addresses::CONFIG_BANK, addresses::MODE_REGISTER, mode)?;
        Ok(())
    }

    /// Set the slave device to sync audio
    pub fn audio_sync(&mut self, yes: bool) -> Result<(), I2cError> {
        self.write_register(
            addresses::CONFIG_BANK,
            addresses::AUDIOSYNC,
            if yes { 1 } else { 0 },
        )?;
        Ok(())
    }

    /// Set the device to sleep
    pub fn sleep(&mut self, yes: bool) -> Result<(), I2cError> {
        self.write_register(
            addresses::CONFIG_BANK,
            addresses::SHUTDOWN,
            if yes { 0 } else { 1 },
        )?;
        Ok(())
    }

    fn write_register(&mut self, bank: u8, register: u8, value: u8) -> Result<(), I2cError> {
        self.bank(bank)?;
        self.i2c.write(self.address, &[register, value])?;
        Ok(())
    }

    fn bank(&mut self, bank: u8) -> Result<(), I2cError> {
        self.i2c
            .write(self.address, &[addresses::BANK_ADDRESS, bank])?;
        Ok(())
    }
}

/// See the [data sheet](https://www.lumissil.com/assets/pdf/core/IS31FL3731_DS.pdf)
/// for more information on registers.
pub mod addresses {
    pub const MODE_REGISTER: u8 = 0x00;
    pub const FRAME: u8 = 0x01;
    pub const AUTOPLAY1: u8 = 0x02;
    pub const AUTOPLAY2: u8 = 0x03;
    pub const BLINK: u8 = 0x05;
    pub const AUDIOSYNC: u8 = 0x06;
    pub const BREATH1: u8 = 0x08;
    pub const BREATH2: u8 = 0x09;
    pub const SHUTDOWN: u8 = 0x0A;
    pub const GAIN: u8 = 0x0B;
    pub const ADC: u8 = 0x0C;

    pub const CONFIG_BANK: u8 = 0x0B;
    pub const BANK_ADDRESS: u8 = 0xFD;

    pub const PICTURE_MODE: u8 = 0x00;
    pub const AUTOPLAY_MODE: u8 = 0x08;
    pub const AUDIOPLAY_MODE: u8 = 0x18;

    pub const ENABLE_OFFSET: u8 = 0x00;
    pub const BLINK_OFFSET: u8 = 0x12;
    pub const COLOR_OFFSET: u8 = 0x24;
}

#[derive(Clone, Copy, Debug)]
pub enum Error<I2cError> {
    I2cError(I2cError),
    InvalidLocation(u8),
    InvalidFrame(u8),
}

impl<E> From<E> for Error<E> {
    fn from(error: E) -> Self {
        Error::I2cError(error)
    }
}
