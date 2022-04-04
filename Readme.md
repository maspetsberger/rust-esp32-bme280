# Rust on ESP32 with BME280 sensor

This repository holds a minimal example for reading the BME280 sensor on an ESP32 using Rust instead of the C++/Arduino.

## Components used in this example

* [Waveshare ESP32-C3](https://www.waveshare.com/wiki/ESP-C3-32S-Kit)
* Bosch BME280 sensor on a breakout-board (can't remember which one exactly)

Unlike the other ESP32 chips, which are built on the Xtensa architecture, the ESP32-C3 is using a RISC-V instruction set. Therefore, the latest Rust nightly chain on llvm can build binaries directly for the ESP32-C3. The specific Waveshare chip listed above does have only 2MB flash, so it does need some customization here and there.

I am not sure what the default I2C pins are on the board I used, so I ended up to manually configure GPIO 8 and 9 for SDA and SCL respectively. According to the docu you could use any GPIO for this. The BME's VIN and GND is connected to the board's `3V3` and `GND` respectively.

## How to build

The example was derived from the [esp-idf-template](https://github.com/esp-rs/esp-idf-template), it holds general build instructions for the various ESP32 chips and how to get the rust nightly chain for RISC-V, or the Xtensa llvm fork for all others.

Specifically for ESP32-C3:

1. `cargo build` (or `cargo build --release`...)
1. `espflash ./target/risv32imc-esp-espidf/debug/rust-esp32-bme280` (or `.../release/...` or `cargo flash`)
1. `espmonitor /dev/ttyUSB0` (or whatever usb port the board is connected to)

Note, the Waveshare ESP32-C3 board only has 2MB flash on board. The 1.3.0 release of [espflash](https://github.com/esp-rs/espflash) is not yet compatible with this, but the latest master is.

## Noteworthy

I stumbled over the following issues throughout development:

* `E (259) spi_flash: Detected size(2048k) smaller than the size in the binary image header(4096k). Probe failed.`
  * Outdated version of `espflash` is used. Either use a more recent version (or master), or switch to `esptool.py`
* `initializing BME280... Guru Meditation Error: Core  0 panic'ed (Illegal instruction). Exception was unhandled.`
  * Double check the SDA and SCL pins, they might need to be swapped in the code.
* `esp-idf-hal` provides several `Delay` providers, they all seem to work.
