use std::{mem, thread, time::Duration};

use anyhow::{bail, Context, Result};
use rppal::i2c::I2c;

fn main() {
    let mut i2c = I2c::new().expect("Failed to init i2c");

    i2c.set_slave_address(0x74)
        .expect("Failed to set slave address");

    let ic = IS31FL3731 {
        address: 0x74,
        i2c,
        frame: 0,
        width: 16,
        height: 9,
    };

    ic.setup().expect("Failed to setup device");

    ic.fill(0, 0, false)
        .expect("Failed to set brightness to full");
}

pub struct IS31FL3731 {
    pub address: u8,
    pub i2c: I2c,
    pub frame: u8,
    pub width: u8,
    pub height: u8,
}

impl IS31FL3731 {
    pub fn fill(&self, brightness: u8, frame: u8, blink: bool) -> Result<()> {
        self.write_bank(frame)?;
        let mut data = vec![brightness; 25].into_boxed_slice();
        for row in 0..6 {
            let _ = mem::replace(data.get_mut(0).unwrap(), addresses::COLOR_OFFSET + row * 24);
            println!("{:?}", data);
            self.i2c
                .block_write(self.address, &data)
                .context("Failed to block write brightness update")?;
        }
        if blink {
            for col in 0..18 {
                self.write_register(frame, addresses::BLINK_OFFSET + col, 0xFF)?;
            }
        }
        Ok(())
    }

    pub fn setup(&mut self) -> Result<()> {
        self.sleep(true)?;
        thread::sleep(Duration::from_millis(10)); // reset
        self.mode(addresses::PICTURE_MODE)?;
        self.frame(0, true)?;
        for frame in 0..8 {
            self.fill(0, frame, false)?;
            for col in 0..18 {
                self.write_register(frame, addresses::ENABLE_OFFSET + col, 0xFF)?;
            }
        }
        self.audio_sync(false)?;
        self.sleep(false)?;
        Ok(())
    }

    pub fn audio_sync(&self, yes: bool) -> Result<()> {
        self.write_register(addresses::CONFIG_BANK, addresses::AUDIOSYNC, yes as u8)
    }

    fn i2c_write_register(&self, register: u8, data: &[u8]) -> Result<()> {
        let mut buf = vec![register];
        buf.extend_from_slice(data);
        self.i2c
            .block_write(self.address, &buf.into_boxed_slice())?;
        Ok(())
    }

    fn write_register(&self, bank: u8, register: u8, value: u8) -> Result<()> {
        self.write_bank(bank)?;
        self.i2c_write_register(register, &[value])?;
        Ok(())
    }

    fn write_bank(&self, data: u8) -> Result<()> {
        Ok(self.i2c_write_register(addresses::BANK_ADDRESS, &[data])?)
    }

    fn frame(&mut self, frame: u8, show: bool) -> Result<()> {
        if frame > 8 {
            bail!("Frame out of range")
        }
        self.frame = frame;
        if show {
            self.write_register(addresses::CONFIG_BANK, addresses::FRAME, frame)?;
        }
        Ok(())
    }

    fn sleep(&self, value: bool) -> Result<()> {
        self.write_register(
            addresses::CONFIG_BANK,
            addresses::SHUTDOWN,
            if value { 0 } else { 1 },
        )?;
        Ok(())
    }

    fn mode(&self, mode: u8) -> Result<()> {
        self.write_register(addresses::CONFIG_BANK, addresses::MODE_REGISTER, mode)
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

    pub const CONFIG_BANK: u8 = 0xFD;
    pub const BANK_ADDRESS: u8 = 0x0B;

    pub const PICTURE_MODE: u8 = 0x00;
    pub const AUTOPLAY_MODE: u8 = 0x08;
    pub const AUDIOPLAY_MODE: u8 = 0x18;

    pub const ENABLE_OFFSET: u8 = 0x00;
    pub const BLINK_OFFSET: u8 = 0x12;
    pub const COLOR_OFFSET: u8 = 0x24;
}
