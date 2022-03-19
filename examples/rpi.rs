use std::{thread, time::Duration};

use anyhow::{bail, Result};
use rppal::i2c::I2c;

fn main() {
    let i2c = I2c::new().expect("Failed to init i2c");

    let mut ic = IS31FL3731 {
        address: 0x74,
        i2c,
        frame: 0,
        width: 16,
        height: 8,
    };

    ic.setup().expect("Failed to setup IC");

    for x in 0..16 {
        for y in 0..8 {
            ic.pixel(x, y, 1).expect("Failed to set pixel value");
            thread::sleep(Duration::from_millis(20));
        }
    }
}

pub struct IS31FL3731 {
    pub address: u16,
    pub i2c: I2c,
    pub frame: u8,
    pub width: u8,
    pub height: u8,
}

impl IS31FL3731 {
    pub fn fill(&self, brightness: u8, blink: Option<bool>, frame: u8) -> Result<()> {
        self.bank(frame)?;
        let payload = &[brightness; 24];
        for row in 0..6 {
            self.i2c
                .block_write(addresses::COLOR_OFFSET + row * 24, payload)?;
        }
        if blink.is_some() {
            let data = if blink.unwrap() { 1 } else { 0 } * 0xFF;
            for col in 0..18 {
                self.write_register(frame, addresses::BLINK_OFFSET + col, data)?;
            }
        }
        Ok(())
    }

    pub fn setup(&mut self) -> Result<()> {
        self.sleep(true)?;
        thread::sleep(Duration::from_millis(10));
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

    pub fn set_address(&mut self, address: u16) {
        self.address = address;
    }

    pub fn pixel(&self, x: u8, y: u8, brightness: u8) -> Result<()> {
        if x > self.width {
            bail!("pixel width out of range")
        }
        if y > self.height {
            bail!("pixel height out of range")
        }
        let pixel = if x >= 8 {
            (x - 6) * 16 - (y + 1)
        } else {
            (x + 1) * 16 + (7 - y)
        };
        self.write_register(self.frame, addresses::COLOR_OFFSET + pixel, brightness)?;
        Ok(())
    }

    pub fn frame(&mut self, frame: u8) -> Result<()> {
        self.frame = frame;
        self.write_register(addresses::CONFIG_BANK, addresses::FRAME, frame)?;
        Ok(())
    }

    pub fn reset(&self) -> Result<()> {
        self.sleep(true)?;
        thread::sleep(Duration::from_millis(10));
        self.sleep(false)?;
        Ok(())
    }

    fn write_register(&self, bank: u8, register: u8, value: u8) -> Result<()> {
        self.bank(bank)?;
        self.i2c.block_write(register, &[value])?;
        Ok(())
    }

    fn bank(&self, bank: u8) -> Result<()> {
        self.i2c.block_write(addresses::BANK_ADDRESS, &[bank])?;
        Ok(())
    }

    fn mode(&self, mode: u8) -> Result<()> {
        self.write_register(addresses::CONFIG_BANK, addresses::MODE_REGISTER, mode)?;
        Ok(())
    }

    fn audio_sync(&self, yes: bool) -> Result<()> {
        self.write_register(
            addresses::CONFIG_BANK,
            addresses::AUDIOSYNC,
            if yes { 1 } else { 0 },
        )?;
        Ok(())
    }

    fn sleep(&self, yes: bool) -> Result<()> {
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
