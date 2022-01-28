#![no_std]

use embedded_hal::blocking::i2c::{Write, WriteRead};

pub use crate::error::Error;
use crate::register::{Bank0, Bank1, Bank2, Bank3, Register};

mod error;
mod register;

/// Unique device identifier for the ICM-42670
const WHOAMI: u8 = 0x67;

/// I²C slave addresses, determined by the logic level of pin AP_AD0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Address {
    /// AP_AD0 = 0
    Primary   = 0x68,
    /// AP_AD0 = 1
    Secondary = 0x69,
}

/// Enum describing possible ranges of the Accelerometer
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AccelRange {
    /// ±2G
    G2,
    /// ±4G
    G4,
    /// ±8G
    G8,
    /// ±16G
    G16,
}

impl AccelRange {
    fn as_float(&self) -> f32 {
        use AccelRange::*;

        // Values taken from Table 2 of the data sheet
        match self {
            G2 => 16_384.0,
            G4 => 8_192.0,
            G8 => 4_096.0,
            G16 => 2_048.0,
        }
    }
}

/// Enum describing possible ranges of the Gyro
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GyroRange {
    /// ±250 deg/sec
    Deg250,
    /// ±500 deg/sec
    Deg500,
    /// ±1000 deg/sec
    Deg1000,
    /// ±2000 deg/sec
    Deg2000,
}

impl GyroRange {
    fn as_float(&self) -> f32 {
        use GyroRange::*;

        // Values taken from Table 1 of the data sheet
        match self {
            Deg250 => 131.0,
            Deg500 => 65.5,
            Deg1000 => 32.8,
            Deg2000 => 16.4,
        }
    }
}

/// ICM-42670 driver
#[derive(Debug)]
pub struct Icm42670<I2C> {
    /// Underlying I²C peripheral
    i2c: I2C,
    /// I²C slave address to use
    address: Address,
}

impl<I2C, E> Icm42670<I2C>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    pub fn new(i2c: I2C, address: Address) -> Result<Self, Error<E>> {
        let mut me = Self { i2c, address };

        if me.device_id()? != WHOAMI {
            return Err(Error::BadChip);
        }

        Ok(me)
    }

    /// Read the ID of the connected device
    pub fn device_id(&mut self) -> Result<u8, Error<E>> {
        self.set_bank(0)?;
        let device_id = self.read_reg(&Bank0::WHO_AM_I)?;

        Ok(device_id)
    }

    /// Read a register at the provided address
    fn read_reg(&mut self, reg: &dyn Register) -> Result<u8, Error<E>> {
        let mut buffer = [0u8];
        self.i2c
            .write_read(self.address as u8, &[reg.addr()], &mut buffer)?;

        Ok(buffer[0])
    }

    /// Set a register at the provided address to a given value
    fn write_reg(&mut self, reg: &dyn Register, value: u8) -> Result<(), Error<E>> {
        if reg.read_only() {
            return Err(Error::WriteToReadOnly);
        }

        self.i2c.write(self.address as u8, &[reg.addr(), value])?;

        Ok(())
    }

    /// Update the register at the provided address
    ///
    /// Rather than overwriting any active bits in the register, we first read
    /// in its current value and then update it accordingly using the given
    /// value and mask before writing back the desired value.
    fn update_reg(&mut self, reg: &dyn Register, value: u8, mask: u8) -> Result<(), Error<E>> {
        let current = self.read_reg(reg)?;
        let value = (current & !mask) | (value & mask);

        self.write_reg(reg, value)?;

        Ok(())
    }
}
