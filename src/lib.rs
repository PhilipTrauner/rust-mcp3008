//! `rust-mcp3008` is a rewrite of the excellent [Adafruit_Python_MCP3008](https://github.com/adafruit/Adafruit_Python_MCP3008) Python library in Rust.

#[cfg(test)]
mod tests {
    use super::Mcp3008;
    use std::path::Path;
    use std::env;

    #[test]
    fn mcp3008_read_adc() {
        let spi_dev_path = "/dev/spidev0.0";

        if cfg!(target_os = "linux") {
            if Path::new(&spi_dev_path).exists() {
                let mut mcp3008 = Mcp3008::new(spi_dev_path).unwrap();

                mcp3008.read_adc(0).unwrap();

                if let Ok(_) = mcp3008.read_adc(8) {
                    panic!("read from adc > 7");
                }
            } else {
                if let Ok(_) = env::var("TRAVIS_RUST_VERSION") {
                    println!("can't mock spi interface on travis, passing test...");
                } else {
                    panic!("can not test on current setup (no spi interface)");
                }
            }
        } else {
            panic!("can not test on current setup (unsupported os)");
        }
    }
}

#[cfg(target_os = "linux")]
extern crate spidev;

use std::io;
use std::fmt;
use std::error::Error;

#[cfg(target_os = "linux")]
use spidev::{SPI_MODE_0, Spidev, SpidevOptions, SpidevTransfer};

#[derive(Debug)]
pub enum Mcp3008Error {
    SpidevError(io::Error),
    AdcOutOfRangeError(u8),
    UnsupportedOSError,
}

impl fmt::Display for Mcp3008Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Mcp3008Error::SpidevError(ref err) => err.fmt(f),
            Mcp3008Error::AdcOutOfRangeError(adc_number) => {
                write!(f, "invalid adc number ({})", adc_number)
            }
            Mcp3008Error::UnsupportedOSError => write!(f, "unsupported os"),
        }
    }
}

impl Error for Mcp3008Error {
    fn description(&self) -> &str {
        match *self {
            Mcp3008Error::SpidevError(ref err) => err.description(),
            Mcp3008Error::AdcOutOfRangeError(_) => "invalid adc number",
            Mcp3008Error::UnsupportedOSError => "unsupported os",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            Mcp3008Error::SpidevError(ref err) => Some(err),
            Mcp3008Error::AdcOutOfRangeError(_) => None,
            Mcp3008Error::UnsupportedOSError => None,
        }
    }
}

impl From<io::Error> for Mcp3008Error {
    fn from(err: io::Error) -> Mcp3008Error {
        Mcp3008Error::SpidevError(err)
    }
}

pub struct Mcp3008 {
    #[cfg(target_os = "linux")]
    spi: Spidev,
}

/// Provides access to a MCP3008 A/D converter.
/// # Example
///
/// ```rust
/// extern crate mcp3008;
///
/// use mcp3008::Mcp3008;
///
/// fn main() {
///     if let Ok(mut mcp3008) = Mcp3008::new("/dev/spidev0.0") {
///         println!("{}", mcp3008.read_adc(0).unwrap());
///     }
/// }
/// ```
impl Mcp3008 {
    /// Constructs a new `Mcp3008`.
    #[cfg(target_os = "linux")]
    pub fn new(spi_dev_path: &str) -> Result<Mcp3008, Mcp3008Error> {
        let options = SpidevOptions::new()
            .max_speed_hz(1_000_000)
            .mode(SPI_MODE_0)
            .lsb_first(false)
            .build();

        let mut spi = Spidev::open(spi_dev_path.to_string())?;

        match spi.configure(&options) {
            Ok(_) => Ok(Mcp3008 { spi: spi }),
            Err(err) => Err(Mcp3008Error::SpidevError(err)),
        }
    }

    #[cfg(not(target_os = "linux"))]
    pub fn new(_spi_dev_path: &str) -> Result<Mcp3008, Mcp3008Error> {
        Err(Mcp3008Error::UnsupportedOSError)
    }

    #[cfg(target_os = "linux")]
    pub fn read_adc(&mut self, adc_number: u8) -> Result<u16, Mcp3008Error> {
        match adc_number {
            0...7 => {
                // Start bit, single channel read
                let mut command: u8 = 0b11 << 6;
                command |= (adc_number & 0x07) << 3;
                // Bottom 3 bits of command are 0, this is to account for the
                // extra clock to do the conversion, and the low null bit returned
                // at the start of the response.

                let tx_buf = [command, 0x0, 0x0];
                let mut rx_buf = [0_u8; 3];

                // Marked as own scope so that rx_buf isn't borrowed
                // anymore after the transfer() call
                {
                    let mut transfer = SpidevTransfer::read_write(&tx_buf, &mut rx_buf);

                    self.spi.transfer(&mut transfer)?;
                }

                let mut result = (rx_buf[0] as u16 & 0x01) << 9;
                result |= (rx_buf[1] as u16 & 0xFF) << 1;
                result |= (rx_buf[2] as u16 & 0x80) >> 7;

                Ok(result & 0x3FF)
            }
            _ => Err(Mcp3008Error::AdcOutOfRangeError(adc_number)),
        }
    }

    #[cfg(not(target_os = "linux"))]
    pub fn read_adc(&mut self, _adc_number: u8) -> Result<u16, Mcp3008Error> {
        Err(Mcp3008Error::UnsupportedOSError)
    }
}
