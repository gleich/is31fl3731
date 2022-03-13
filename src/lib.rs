#![no_std]

use embedded_hal::blocking::i2c::WriteRead;

#[derive(Clone, Copy, Debug)]
pub enum Error<I2cError> {
    I2cError(I2cError),
}

pub struct Driver<I2C> {
    i2c: I2C,
}

// impl<I2C, I2cError> Driver<I2C>
// where
//     I2C: WriteRead<Error = I2cError>,
// {
//     pub fn new(i2c: I2C) -> Result<Driver<I2C>, Error<I2cError>> {
//         let mut driver = Driver { i2c };
//     }

//     fn get_id(&mut self) -> Result<u8, Error<I2cError>> {
//         let mut buffer = [0u8; 1];
//     }
// }

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
