# is31fl3731 driver

[![Crates.io](https://img.shields.io/crates/v/is31fl3731)](https://crates.io/crates/is31fl3731)
[![Crates.io](https://img.shields.io/crates/d/is31fl3731)](https://crates.io/crates/is31fl3731)
[![docs.rs](https://img.shields.io/docsrs/is31fl3731)](https://docs.rs/is31fl3731/latest/is31fl3731/)

[![lint](https://github.com/gleich/is31fl3731/actions/workflows/lint.yml/badge.svg)](https://github.com/gleich/is31fl3731/actions/workflows/lint.yml)
[![build](https://github.com/gleich/is31fl3731/actions/workflows/build.yml/badge.svg)](https://github.com/gleich/is31fl3731/actions/workflows/build.yml)

Driver for [Lumissil Microsystem's IS31FL3731 integrated circuit](https://www.lumissil.com/assets/pdf/core/IS31FL3731_DS.pdf). Some of the major features of this library are:

1. Use of embedded HAL traits (works with any embedded device that supports the required traits). This means that this driver is platform agnostic.
2. Library features (only turn on what devices you need to save compiled binary space).
3. [Examples](./examples) on how to use this driver. Right now there is only an example on how to use this crate with a raspberry pi. If you're looking for an embedded example check out my [random_matrix github repo](https://github.com/gleich/random_matrix) which uses this driver on the raspberry pi pico (rp2040 microcontroller).

## Install

To install this driver in your project add the following line to your `Cargo.toml`'s `dependencies` table:

```toml
is31fl3731 = "1.0.1"
```

By default this version will only contain the core driver. To use a preconfigured device, such as the [Adafruit CharliePlex LED Matrix Bonnet](https://www.adafruit.com/product/3467), you would need to change this line to include that device:

```toml
is31fl3732 = { version = "1.0.1", features = ["charlie_bonnet"] }
```

## Functionality & Plans

Currently this library only supports some basic functions of the matrix (e.g. setup, fill, pixels). A few other features need to be implemented:

- [ ] autoplay
- [ ] fade
- [ ] audio_play
- [ ] blink

## Inspiration

This driver is a port of [adafruit's driver for the is31fl3731](https://github.com/adafruit/Adafruit_CircuitPython_IS31FL3731) in the rust programming language.
