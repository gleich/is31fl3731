use is31fl3731::Driver;
use rppal::i2c::I2c;

fn main() {
    let i2c = I2c::new().expect("Failed to load i2c bus");
    let delay = rppal::hal::Delay;
    let mut ic = Driver {
        i2c,
        delay,
        address: 0x74,
        frame: 0,
        width: 16,
        height: 8,
    };
    ic.setup().expect("Failed to setup IC");
    ic.fill(10, None, 0).expect("Failed to fill license");
}
