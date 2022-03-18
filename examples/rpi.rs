use std::{thread, time::Duration};

use anyhow::Result;
use rppal::i2c::I2c;

fn main() {
    let i2c = I2c::new().expect("Failed to init i2c");

    let mut ic = IS31FL3731 {
        address: 0x74,
        i2c,
        frame: 0,
        width: 16,
        height: 9,
    };

    ic.setup().expect("Failed to setup IC");

    ic.fill(10, None, 0)
        .expect("Failed to fill display with brightness of 0");
}

pub struct IS31FL3731 {
    pub address: u8,
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
                self.register(frame, addresses::BLINK_OFFSET + col, data)?;
            }
        }
        Ok(())
    }

    pub fn setup(&mut self) -> Result<()> {
        self.i2c
            .set_slave_address(0x74)
            .expect("Failed to set slave address");
        self.sleep(true)?;
        thread::sleep(Duration::from_millis(10));
        self.mode(addresses::PICTURE_MODE)?;
        self.frame(0)?;
        for frame in 0..8 {
            self.fill(0, Some(false), frame)?;
            for col in 0..18 {
                self.register(frame, addresses::ENABLE_OFFSET + col, 0xFF)?;
            }
        }
        self.audio_sync(false)?;
        self.sleep(false)?;
        Ok(())
    }

    fn register(&self, bank: u8, register: u8, value: u8) -> Result<()> {
        self.bank(bank)?;
        self.i2c_write_reg(register, &[value])?;
        Ok(())
    }

    // TESTED: works
    fn bank(&self, bank: u8) -> Result<()> {
        self.i2c_write_reg(addresses::BANK_ADDRESS, &[bank])?;
        Ok(())
    }

    fn mode(&self, mode: u8) -> Result<()> {
        self.register(addresses::CONFIG_BANK, addresses::MODE_REGISTER, mode)?;
        Ok(())
    }

    pub fn frame(&self, frame: u8) -> Result<()> {
        self.register(addresses::CONFIG_BANK, addresses::FRAME, frame)?;
        Ok(())
    }

    fn audio_sync(&self, yes: bool) -> Result<()> {
        self.register(
            addresses::CONFIG_BANK,
            addresses::AUDIOSYNC,
            if yes { 1 } else { 0 },
        )?;
        Ok(())
    }

    fn sleep(&self, yes: bool) -> Result<()> {
        self.register(
            addresses::CONFIG_BANK,
            addresses::SHUTDOWN,
            if yes { 0 } else { 1 },
        )?;
        Ok(())
    }

    fn i2c_write_reg(&self, reg: u8, data: &[u8]) -> Result<()> {
        self.i2c.block_write(reg, data)?;
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
