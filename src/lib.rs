#![no_std]

use core::marker::PhantomData;

use embedded_hal::blocking::i2c;

pub struct IS31FL3731<I2C> {
    i2c: PhantomData<I2C>,
    address: u8,
}

impl<I2C, E> IS31FL3731<I2C>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    pub fn new(_i2c: &I2C, address: u8) -> Result<Self, E> {
        let is31fl3731 = IS31FL3731 {
            i2c: PhantomData,
            address,
        };

        Ok(is31fl3731)
    }
}
