use crate::IS31FL3731;

pub struct CharlieBonnet;

impl CharlieBonnet {
    pub fn configure<I2C, DEL>(i2c: I2C, delay: DEL) -> IS31FL3731<I2C, DEL> {
        IS31FL3731 {
            i2c,
            delay,
            address: 0x74,
            frame: 0,
            width: 16,
            height: 8,
            calc_pixel: |x: u8, y: u8| -> u8 {
                if x >= 8 {
                    (x - 6) * 16 - (y + 1)
                } else {
                    (x + 1) * 16 + (7 - y)
                }
            },
        }
    }
}
