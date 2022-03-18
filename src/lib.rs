#![no_std]

// #[derive(Clone, Copy, Debug)]
// pub enum Error<I2cError> {
//     I2cError(I2cError),
// }
//
// pub struct Driver<I2C> {
//     i2c: I2C,
// }

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
