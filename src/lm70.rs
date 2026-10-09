use embedded_hal::spi::{Operation, SpiDevice};

use crate::{conversion, Error, Lm70};

const CONTINUOUS_MODE: [u8; 2] = [0x00, 0x00];
const SHUTDOWN_MODE: [u8; 2] = [0x00, 0xff];

impl<SPI> Lm70<SPI> {
    /// Creates a new LM70 driver from an SPI device.
    pub const fn new(spi: SPI) -> Self {
        Self { spi }
    }

    /// Destroys the driver and returns the SPI device.
    pub fn destroy(self) -> SPI {
        self.spi
    }
}

impl<SPI> Lm70<SPI>
where
    SPI: SpiDevice<u8>,
{
    /// Reads the current temperature in degrees Celsius.
    ///
    /// The result has the LM70 native resolution of 0.25 degrees Celsius.
    pub fn read_temperature(&mut self) -> Result<f32, Error<SPI::Error>> {
        let raw = self.read_word()?;
        if !conversion::is_temperature(raw) {
            return Err(Error::InvalidData(raw));
        }
        Ok(conversion::convert_temperature(raw))
    }

    /// Enters shutdown mode.
    ///
    /// In shutdown mode the serial output exposes the manufacturer/device ID
    /// instead of temperature data.
    pub fn shutdown(&mut self) -> Result<(), Error<SPI::Error>> {
        self.write_configuration(&SHUTDOWN_MODE)
    }

    /// Leaves shutdown mode and resumes continuous temperature conversion.
    pub fn continuous(&mut self) -> Result<(), Error<SPI::Error>> {
        self.write_configuration(&CONTINUOUS_MODE)
    }

    /// Reads the 11-bit manufacturer/device identification value.
    ///
    /// The LM70 must first be placed in shutdown mode with [`Lm70::shutdown`].
    pub fn read_manufacturer_id(&mut self) -> Result<u16, Error<SPI::Error>> {
        let raw = self.read_word()?;
        if !conversion::is_manufacturer_id(raw) {
            return Err(Error::InvalidData(raw));
        }
        Ok(conversion::convert_manufacturer_id(raw))
    }

    fn read_word(&mut self) -> Result<u16, Error<SPI::Error>> {
        let mut bytes = [0; 2];
        self.spi.read(&mut bytes).map_err(Error::Spi)?;
        Ok(u16::from_be_bytes(bytes))
    }

    fn write_configuration(&mut self, configuration: &[u8; 2]) -> Result<(), Error<SPI::Error>> {
        let mut discarded_register = [0; 2];
        let mut operations = [
            Operation::Read(&mut discarded_register),
            Operation::Write(configuration),
        ];
        self.spi.transaction(&mut operations).map_err(Error::Spi)
    }
}
