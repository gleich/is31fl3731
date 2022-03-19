use std::{thread, time::Duration};

use is31fl3731::IS31FL3731;
use rppal::i2c::I2c;

fn main() {
    let mut ic = IS31FL3731 {
        i2c: I2c::new().expect("Failed to load i2c bus"),
        delay: rppal::hal::Delay,
        address: 0x74,
        frame: 0,
        width: 16,
        height: 8,
    };
    ic.setup().expect("Failed to setup IC");

    for x in 0..16 {
        for y in 0..8 {
            ic.pixel(x, y, 1).expect("Failed to set pxiel value");
            thread::sleep(Duration::from_millis(20));
        }
    }
}
