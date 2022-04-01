use crate::error::Error;

pub(crate) trait Bitfield {
    const BITMASK: u8;

    /// Bit value of a discriminant, shifted to the correct position if
    /// necessary
    fn bits(self) -> u8;
}

/// I²C slave addresses, determined by the logic level of pin `AP_AD0`
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Address {
    /// `AP_AD0` pin == 0
    Primary   = 0x68,
    /// `AP_AD0` pin == 1
    Secondary = 0x69,
}

/// Configurable ranges of the Accelerometer
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AccelRange {
    /// ±2G
    G2  = 3,
    /// ±4G
    G4  = 2,
    /// ±8G
    G8  = 1,
    /// ±16G
    G16 = 0,
}

impl AccelRange {
    /// Sensitivity scale factor
    pub fn scale_factor(&self) -> f32 {
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

impl Bitfield for AccelRange {
    const BITMASK: u8 = 0b0110_0000;

    fn bits(self) -> u8 {
        // `ACCEL_UI_FS_SEL` occupies bits 6:5 in the register
        (self as u8) << 5
    }
}

impl Default for AccelRange {
    fn default() -> Self {
        Self::G16
    }
}

impl TryFrom<u8> for AccelRange {
    type Error = Error<()>;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use AccelRange::*;

        match value {
            0 => Ok(G16),
            1 => Ok(G8),
            2 => Ok(G4),
            3 => Ok(G2),
            _ => Err(Error::InvalidDiscriminant),
        }
    }
}

/// Configurable ranges of the Gyroscope
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GyroRange {
    /// ±250 deg/sec
    Deg250  = 3,
    /// ±500 deg/sec
    Deg500  = 2,
    /// ±1000 deg/sec
    Deg1000 = 1,
    /// ±2000 deg/sec
    Deg2000 = 0,
}

impl GyroRange {
    /// Sensitivity scale factor
    pub fn scale_factor(&self) -> f32 {
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

impl Bitfield for GyroRange {
    const BITMASK: u8 = 0b0110_0000;

    fn bits(self) -> u8 {
        // `GYRO_UI_FS_SEL` occupies bits 6:5 in the register
        (self as u8) << 5
    }
}

impl Default for GyroRange {
    fn default() -> Self {
        Self::Deg2000
    }
}

impl TryFrom<u8> for GyroRange {
    type Error = Error<()>;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use GyroRange::*;

        match value {
            0 => Ok(Deg2000),
            1 => Ok(Deg1000),
            2 => Ok(Deg500),
            3 => Ok(Deg250),
            _ => Err(Error::InvalidDiscriminant),
        }
    }
}

/// Configurable power modes of the IMU
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PowerMode {
    /// Gyroscope: OFF, Accelerometer: OFF
    Sleep           = 0b0000,
    /// Gyroscope: DRIVE ON, Accelerometer: OFF
    Standby         = 0b0100,
    /// Gyroscope: OFF, Accelerometer: DUTY-CYCLED
    AccelLowPower   = 0b0010,
    /// Gyroscope: OFF, Accelerometer: ON
    AccelLowNoise   = 0b0011,
    /// Gyroscope: ON, Accelerometer: OFF
    GyroLowNoise    = 0b1100,
    /// Gyroscope: ON, Accelerometer: ON
    SixAxisLowNoise = 0b1111,
}

impl Bitfield for PowerMode {
    const BITMASK: u8 = 0b0000_1111;

    fn bits(self) -> u8 {
        // `GYRO_MODE` occupies bits 3:2 in the register
        // `ACCEL_MODE` occupies bits 1:0 in the register
        self as u8
    }
}

impl Default for PowerMode {
    fn default() -> Self {
        PowerMode::Sleep
    }
}

impl TryFrom<u8> for PowerMode {
    type Error = Error<()>;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use PowerMode::*;

        match value {
            0b0000 => Ok(Sleep),
            0b0100 => Ok(Standby),
            0b0010 => Ok(AccelLowPower),
            0b0011 => Ok(AccelLowNoise),
            0b1100 => Ok(GyroLowNoise),
            0b1111 => Ok(SixAxisLowNoise),
            _ => Err(Error::InvalidDiscriminant),
        }
    }
}
