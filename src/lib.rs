#![no_std]

use core::marker::PhantomData;

use embedded_hal::blocking::i2c;

pub struct IS31FL3731<I2C> {
    i2c: PhantomData<I2C>,
    address: u8,
    width: u32,
    height: u32,
}

impl<I2C, E> IS31FL3731<I2C>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    pub fn new(_i2c: &I2C, address: u8, width: u32, height: u32) -> Result<Self, E> {
        let is31fl3731 = IS31FL3731 {
            i2c: PhantomData,
            address,
            width,
            height,
        };

        Ok(is31fl3731)
    }
}

enum Register {
    MODE = 0x00,
    FRAME = 0x01,
    AUTOPLAY1 = 0x02,
    AUTOPLAY2 = 0x03,
    BLINK = 0x05,
    AUDIOSYNC = 0x06,
    BREATH1 = 0x08,
    BREATH2 = 0x09,
    SHUTDOWN = 0x0A,
    GAIN = 0x0B,
    ADC = 0x0C,
}

enum Bank {
    CONFIG = 0x0B,
    ADDRESS = 0xFD,
}

enum Mode {
    PICTURE = 0x00,
    AUTOPLAY = 0x08,
    AUDIOPLAY = 0x18,
}

enum Offset {
    ENABLE = 0x00,
    BLINK = 0x12,
    COLOR = 0x24,
}
