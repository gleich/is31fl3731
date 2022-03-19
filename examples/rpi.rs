use std::{thread, time::Duration};

use is31fl3731::{devices::CharlieBonnet, IS31FL3731};
use rppal::{hal, i2c::I2c};

fn main() {
    let mut ic = CharlieBonnet::configure(I2c::new().expect("Failed to load i2c bus"), hal::Delay);
    ic.setup().expect("Failed to setup IC");

    for x in 0..16 {
        for y in 0..8 {
            ic.pixel(x, y, 1).expect("Failed to set pxiel value");
            thread::sleep(Duration::from_millis(20));
        }
    }
}
