# lm70-rs

Platform-independent, `no_std` Rust driver for the Texas Instruments LM70 temperature sensor, built on the blocking [`embedded-hal`](https://crates.io/crates/embedded-hal) 1.0 SPI traits.

## Features

- Reads temperatures with the LM70 native 0.25 degrees Celsius resolution.
- Correctly decodes the signed 11-bit two's-complement temperature field.
- Enters shutdown mode and resumes continuous conversion.
- Reads the manufacturer/device identification register while shut down.
- Validates the LM70 register-identification bits.
- Uses `SpiDevice`, so chip-select remains owned by the HAL implementation.
- Has no allocator, operating-system, or standard-library dependency.

## Device and protocol

The LM70 is a local temperature sensor with an SPI/MICROWIRE-compatible serial interface. Refer to the [LM70 data sheet](https://www.ti.com/lit/ds/symlink/lm70.pdf) for electrical limits, timing, pinout, and package information.

A normal read clocks one 16-bit word, most-significant byte first:

| Bits | Meaning |
| --- | --- |
| D15-D5 | Signed 11-bit temperature in two's-complement format |
| D4-D2 | `111` for temperature data, `000` for manufacturer/device ID |
| D1-D0 | Undefined/tri-stated by the device |

The conversion implemented by this crate is:

```text
temperature_celsius = ((raw_word as signed 16-bit) >> 5) * 0.25
```

For example, `0x0C9C` represents `25.0 degrees Celsius` and `0xFFFC` represents `-0.25 degrees Celsius`. Bits D1-D0 are ignored.

Writing the configuration register requires one uninterrupted 32-clock transaction. During the first 16 clocks the LM70 outputs its current register; during the following 16 clocks it accepts `0x00FF` for shutdown or `0x0000` for continuous conversion. The driver models this as one `SpiDevice::transaction`, ensuring chip-select stays asserted throughout.

Configure the SPI peripheral according to the timing requirements in the data sheet. The device shifts the most-significant bit first.

## Usage

Add the dependency to your application:

```toml
[dependencies]
lm70 = "0.1"
```

Pass any platform SPI device that implements `embedded_hal::spi::SpiDevice<u8>`:

```rust
use embedded_hal::spi::SpiDevice;
use lm70::{Error, Lm70};

fn sample<SPI>(spi: SPI) -> Result<f32, Error<SPI::Error>>
where
    SPI: SpiDevice<u8>,
{
    let mut sensor = Lm70::new(spi);
    sensor.read_temperature()
}
```

The concrete SPI peripheral, bus-sharing adapter, and chip-select pin come from the HAL for the target microcontroller or operating system.

### Power modes

```rust
# use embedded_hal::spi::SpiDevice;
# use lm70::{Error, Lm70};
# fn power_cycle<SPI>(spi: SPI) -> Result<(), Error<SPI::Error>>
# where SPI: SpiDevice<u8> {
let mut sensor = Lm70::new(spi);

sensor.shutdown()?;
let manufacturer_id = sensor.read_manufacturer_id()?;

sensor.continuous()?;
let temperature_celsius = sensor.read_temperature()?;
# let _ = (manufacturer_id, temperature_celsius);
# Ok(())
# }
```

After `continuous()`, allow the conversion time specified by the data sheet before relying on a newly converted sample.

## Errors

All operations return `lm70::Error<SPI::Error>`:

- `Error::Spi(error)` contains an error from the concrete SPI device.
- `Error::InvalidData(raw)` means D4-D2 did not identify the register expected by the requested operation. The raw 16-bit word is retained for diagnostics.

## Development

Type-check the library and mock-based protocol tests without target hardware:

```console
cargo check --lib
cargo check --tests
```

Run the tests on a host with a working Rust linker:

```console
cargo test
```

The tests use `embedded-hal-mock`; they do not communicate with physical hardware.

## API documentation

Rust API documentation is generated from the `///` and `//!` source comments:

```console
cargo doc --no-deps --open
```

A Doxygen configuration is also included. With Doxygen installed, generate HTML into `target/doxygen/html`:

```console
doxygen Doxyfile
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))
