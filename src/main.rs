use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module

use std::error::Error;
use std::thread;
use std::time::Duration;

use esp_idf_hal::delay;
use esp_idf_hal::i2c;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::units::*;

use bme280::i2c::BME280;

fn main() -> Result<(), Box<dyn Error>> {
    println!("starting...");

    measure_bme()?;

    println!("done.");
    Ok(())
}

fn measure_bme() -> Result<(), Box<dyn Error>> {
    let peripherals = Peripherals::take().unwrap();
    let scl = peripherals.pins.gpio8;
    let sda = peripherals.pins.gpio9;

    // create the i2c bus for the given scl/sda pins
    println!("creating I2C bus...");
    let i2c = i2c::Master::new(
        peripherals.i2c0,
        i2c::MasterPins {
            scl: scl.into_output()?,
            sda: sda.into_input_output()?,
        },
        i2c::config::MasterConfig::new().baudrate(400.kHz().into()),
    )?;

    // there are multiple delay providers, this seems to be most frequently used
    let mut delay = delay::Ets;

    // create the BME280 using the primary I2C address 0x76
    println!("creating BME280...");
    let mut bme280 = BME280::new_primary(i2c);

    // initialize the sensor
    println!("initializing BME280...");
    bme280.init(&mut delay).unwrap();

    // measure temperature, pressure, and humidity
    println!("starting measurements...");
    for _ in 0..100 {
        let measurements = bme280.measure(&mut delay).unwrap();

        println!(
            "Relative Humidity = {:3.2} %,   Temperature = {:3.2} °C,   Pressure = {:4.2} hPa",
            measurements.humidity,
            measurements.temperature,
            measurements.pressure / 100_f32
        );

        // wait one second before next read out.
        thread::sleep(Duration::from_secs(1));
    }

    println!("done measuring.");
    Ok(())
}
