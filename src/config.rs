use crate::error::Error;

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
    /// Attempt to create a discriminant from the provided value
    pub fn try_from<E>(value: u8) -> Result<Self, Error<E>> {
        use AccelRange::*;

        match value {
            0 => Ok(G16),
            1 => Ok(G8),
            2 => Ok(G4),
            3 => Ok(G2),
            _ => Err(Error::InvalidDiscriminant),
        }
    }

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

impl Default for AccelRange {
    fn default() -> Self {
        Self::G16
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
    /// Attempt to create a discriminant from the provided value
    pub fn try_from<E>(value: u8) -> Result<Self, Error<E>> {
        use GyroRange::*;

        match value {
            0 => Ok(Deg2000),
            1 => Ok(Deg1000),
            2 => Ok(Deg500),
            3 => Ok(Deg250),
            _ => Err(Error::InvalidDiscriminant),
        }
    }

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

impl Default for GyroRange {
    fn default() -> Self {
        Self::Deg2000
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

impl Default for PowerMode {
    fn default() -> Self {
        PowerMode::Sleep
    }
}

impl PowerMode {
    /// Attempt to create a discriminant from the provided value
    pub fn try_from<E>(value: u8) -> Result<Self, Error<E>> {
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
