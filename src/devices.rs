#[allow(unused_imports)]
use crate::{Error, IS31FL3731};
#[allow(unused_imports)]
use embedded_hal::blocking::delay::DelayMs;
#[allow(unused_imports)]
use embedded_hal::blocking::i2c::Write;

#[cfg(feature = "charlie_bonnet")]
pub struct CharlieBonnet;
#[cfg(feature = "charlie_wing")]
pub struct CharlieWing;
#[cfg(feature = "keybow_2040")]
pub struct Keybow2040<I2C, DEL> {
    device: IS31FL3731<I2C, DEL>,
}
#[cfg(feature = "led_shim")]
pub struct LEDShim<I2C, DEL> {
    device: IS31FL3731<I2C, DEL>,
}
#[cfg(feature = "matrix")]
pub struct Matrix;
#[cfg(feature = "rgb_matrix_5x5")]
pub struct RGBMatrix5x5<I2C, DEL> {
    device: IS31FL3731<I2C, DEL>,
}
#[cfg(feature = "scroll_phat_hd")]
pub struct ScrollPhatHD;

#[cfg(feature = "charlie_bonnet")]
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

#[cfg(feature = "charlie_wing")]
impl CharlieWing {
    pub fn configure<I2C, DEL>(i2c: I2C, delay: DEL) -> IS31FL3731<I2C, DEL> {
        IS31FL3731 {
            i2c,
            delay,
            address: 0x74,
            frame: 0,
            width: 15,
            height: 7,
            calc_pixel: |x: u8, y: u8| -> u8 {
                let mut x = x;
                let mut y = y;
                if x > 7 {
                    x -= 15;
                    y += 8;
                } else {
                    y = 7 - y
                }
                x * 16 + y
            },
        }
    }
}

#[cfg(feature = "keybow_2040")]
impl<I2C, DEL, I2cError> Keybow2040<I2C, DEL>
where
    I2C: Write<Error = I2cError>,
    DEL: DelayMs<u8>,
{
    pub fn configure(i2c: I2C, delay: DEL) -> Self {
        Self {
            device: IS31FL3731 {
                i2c,
                delay,
                address: 0x74,
                frame: 0,
                width: 16,
                height: 3,
                calc_pixel: |x: u8, y: u8| -> u8 {
                    let lookup = [
                        [120, 88, 104],
                        [136, 40, 72],
                        [112, 80, 96],
                        [128, 32, 64],
                        [121, 89, 105],
                        [137, 41, 73],
                        [113, 81, 97],
                        [129, 33, 65],
                        [122, 90, 106],
                        [138, 25, 74],
                        [114, 82, 98],
                        [130, 17, 66],
                        [123, 91, 107],
                        [139, 26, 75],
                        [115, 83, 99],
                        [131, 18, 67],
                    ];
                    lookup[x as usize][y as usize]
                },
            },
        }
    }

    pub fn pixel_rgb(&mut self, x: u8, y: u8, r: u8, g: u8, b: u8) -> Result<(), Error<I2cError>> {
        let x = (4 * (3 - x)) + y;
        self.device.pixel(x, 0, r)?;
        self.device.pixel(x, 1, g)?;
        self.device.pixel(x, 2, b)?;
        Ok(())
    }
}

#[cfg(feature = "led_shim")]
impl<I2C, DEL, I2cError> LEDShim<I2C, DEL>
where
    I2C: Write<Error = I2cError>,
    DEL: DelayMs<u8>,
{
    pub fn configure(i2c: I2C, delay: DEL) -> Self {
        Self {
            device: IS31FL3731 {
                i2c,
                delay,
                address: 0x75,
                frame: 0,
                width: 28,
                height: 3,
                calc_pixel: |x: u8, y: u8| -> u8 {
                    if y == 0 {
                        if x < 7 {
                            return 118 - x;
                        }
                        if x < 15 {
                            return 141 - x;
                        }
                        if x < 21 {
                            return 106 + x;
                        }
                        if x == 21 {
                            return 14;
                        }
                        return x - 14;
                    }
                    if y == 1 {
                        if x < 2 {
                            return 69 - x;
                        }
                        if x < 7 {
                            return 86 - x;
                        }
                        if x < 12 {
                            return 28 - x;
                        }
                        if x < 14 {
                            return 45 - x;
                        }
                        if x == 14 {
                            return 47;
                        }
                        if x == 15 {
                            return 41;
                        }
                        if x < 21 {
                            return x + 9;
                        }
                        if x == 21 {
                            return 95;
                        }
                        if x < 26 {
                            return x + 67;
                        }
                        return x + 50;
                    }

                    if x == 0 {
                        return 85;
                    }
                    if x < 7 {
                        return 102 - x;
                    }
                    if x < 11 {
                        return 44 - x;
                    }
                    if x == 14 {
                        return 63;
                    }
                    if x < 17 {
                        return 42 + x;
                    }
                    if x < 21 {
                        return x + 25;
                    }
                    if x == 21 {
                        return 111;
                    }
                    if x < 27 {
                        return x + 83;
                    }

                    93
                },
            },
        }
    }

    pub fn pixel_rgb(&mut self, x: u8, r: u8, g: u8, b: u8) -> Result<(), Error<I2cError>> {
        self.device.pixel(x, 0, r)?;
        self.device.pixel(x, 1, g)?;
        self.device.pixel(x, 2, b)?;
        Ok(())
    }
}

#[cfg(feature = "matrix")]
impl Matrix {
    pub fn configure<I2C, DEL>(i2c: I2C, delay: DEL) -> IS31FL3731<I2C, DEL> {
        IS31FL3731 {
            i2c,
            delay,
            address: 0x74,
            frame: 0,
            width: 16,
            height: 9,
            calc_pixel: |x: u8, y: u8| -> u8 { x + y * 16 },
        }
    }
}

#[cfg(feature = "rgb_matrix_5x5")]
impl<I2C, DEL, I2cError> RGBMatrix5x5<I2C, DEL>
where
    I2C: Write<Error = I2cError>,
    DEL: DelayMs<u8>,
{
    pub fn configure(i2c: I2C, delay: DEL) -> Self {
        Self {
            device: IS31FL3731 {
                i2c,
                delay,
                address: 0x75,
                frame: 0,
                width: 25,
                height: 3,
                calc_pixel: |x: u8, y: u8| -> u8 {
                    let lookup = [
                        [118, 69, 85],
                        [117, 68, 101],
                        [116, 84, 100],
                        [115, 83, 99],
                        [114, 82, 98],
                        [132, 19, 35],
                        [133, 20, 36],
                        [134, 21, 37],
                        [112, 80, 96],
                        [113, 81, 97],
                        [131, 18, 34],
                        [130, 17, 50],
                        [129, 33, 49],
                        [128, 32, 48],
                        [127, 47, 63],
                        [125, 28, 44],
                        [124, 27, 43],
                        [123, 26, 42],
                        [122, 25, 58],
                        [121, 41, 57],
                        [126, 29, 45],
                        [15, 95, 111],
                        [8, 89, 105],
                        [9, 90, 106],
                        [10, 91, 107],
                    ];
                    lookup[x as usize][y as usize]
                },
            },
        }
    }

    pub fn pixel_rgb(&mut self, x: u8, y: u8, r: u8, g: u8, b: u8) -> Result<(), Error<I2cError>> {
        let x = x + y * 5;
        self.device.pixel(x, 0, r)?;
        self.device.pixel(x, 1, g)?;
        self.device.pixel(x, 2, b)?;
        Ok(())
    }
}

#[cfg(feature = "scroll_phat_hd")]
impl ScrollPhatHD {
    pub fn configure<I2C, DEL>(i2c: I2C, delay: DEL) -> IS31FL3731<I2C, DEL> {
        IS31FL3731 {
            i2c,
            delay,
            address: 0x74,
            frame: 0,
            width: 17,
            height: 7,
            calc_pixel: |x: u8, y: u8| -> u8 {
                let mut x = x;
                let mut y = y;
                if x <= 8 {
                    x = 8 - x;
                    y = 6 - y;
                } else {
                    x -= 8;
                    y -= 8;
                }
                x * 16 + y
            },
        }
    }
}
