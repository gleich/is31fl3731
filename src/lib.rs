#![no_std]

use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::blocking::i2c::Write;

#[derive(Clone, Copy, Debug)]
pub enum Error<I2cError> {
    I2cError(I2cError),
}

pub struct Driver<I2C, DEL> {
    pub i2c: I2C,
    pub delay: DEL,
    pub address: u8,
    pub frame: u8,
    pub width: u8,
    pub height: u8,
}

impl<I2C, DEL, I2cError> Driver<I2C, DEL>
where
    I2C: Write<Error = I2cError>,
    DEL: DelayMs<u8>,
{
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

    pub fn setup(&mut self) -> Result<(), I2cError> {
        self.sleep(true)?;
        self.delay.delay_ms(10);
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

    pub fn set_address(&mut self, address: u8) {
        self.address = address;
    }

    pub fn frame(&mut self, frame: u8) -> Result<(), I2cError> {
        self.frame = frame;
        self.write_register(addresses::CONFIG_BANK, addresses::FRAME, frame)?;
        Ok(())
    }

    pub fn reset(&mut self) -> Result<(), I2cError> {
        self.sleep(true)?;
        self.delay.delay_ms(10);
        self.sleep(false)?;
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

    fn mode(&mut self, mode: u8) -> Result<(), I2cError> {
        self.write_register(addresses::CONFIG_BANK, addresses::MODE_REGISTER, mode)?;
        Ok(())
    }

    fn audio_sync(&mut self, yes: bool) -> Result<(), I2cError> {
        self.write_register(
            addresses::CONFIG_BANK,
            addresses::AUDIOSYNC,
            if yes { 1 } else { 0 },
        )?;
        Ok(())
    }

    fn sleep(&mut self, yes: bool) -> Result<(), I2cError> {
        self.write_register(
            addresses::CONFIG_BANK,
            addresses::SHUTDOWN,
            if yes { 0 } else { 1 },
        )?;
        Ok(())
    }
}

mod addresses {
    #![allow(dead_code)]
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
