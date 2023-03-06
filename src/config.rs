use crate::error::SensorError;

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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum AccelRange {
    /// ±2G
    G2  = 3,
    /// ±4G
    G4  = 2,
    /// ±8G
    G8  = 1,
    /// ±16G
    #[default]
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

impl TryFrom<u8> for AccelRange {
    type Error = SensorError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use AccelRange::*;

        match value {
            0 => Ok(G16),
            1 => Ok(G8),
            2 => Ok(G4),
            3 => Ok(G2),
            _ => Err(SensorError::InvalidDiscriminant),
        }
    }
}

/// Configurable ranges of the Gyroscope
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum GyroRange {
    /// ±250 deg/sec
    Deg250  = 3,
    /// ±500 deg/sec
    Deg500  = 2,
    /// ±1000 deg/sec
    Deg1000 = 1,
    /// ±2000 deg/sec
    #[default]
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

impl TryFrom<u8> for GyroRange {
    type Error = SensorError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use GyroRange::*;

        match value {
            0 => Ok(Deg2000),
            1 => Ok(Deg1000),
            2 => Ok(Deg500),
            3 => Ok(Deg250),
            _ => Err(SensorError::InvalidDiscriminant),
        }
    }
}

/// Configurable power modes of the IMU
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum PowerMode {
    /// Gyroscope: OFF, Accelerometer: OFF
    #[default]
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

impl TryFrom<u8> for PowerMode {
    type Error = SensorError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use PowerMode::*;

        match value {
            0b0000 => Ok(Sleep),
            0b0100 => Ok(Standby),
            0b0010 => Ok(AccelLowPower),
            0b0011 => Ok(AccelLowNoise),
            0b1100 => Ok(GyroLowNoise),
            0b1111 => Ok(SixAxisLowNoise),
            _ => Err(SensorError::InvalidDiscriminant),
        }
    }
}

/// Accelerometer ODR selection values
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum AccelOdr {
    /// 1.6 kHz (LN mode)
    Hz1600   = 0b0101,
    /// 800 Hz (LN mode
    #[default]
    Hz800    = 0b0110,
    /// 400 Hz (LP or LN mode)
    Hz400    = 0b0111,
    /// 200 Hz (LP or LN mode)
    Hz200    = 0b1000,
    /// 100 Hz (LP or LN mode)
    Hz100    = 0b1001,
    /// 50 Hz (LP or LN mode)
    Hz50     = 0b1010,
    /// 25 Hz (LP or LN mode)
    Hz25     = 0b1011,
    /// 12.5 Hz (LP or LN mode)
    Hz12_5   = 0b1100,
    /// 6.25 Hz (LP mode)
    Hz6_25   = 0b1101,
    /// 3.125 Hz (LP mode)
    Hz3_125  = 0b1110,
    /// 1.5625 Hz (LP mode
    Hz1_5625 = 0b1111,
}

impl AccelOdr {
    pub fn as_f32(self) -> f32 {
        use AccelOdr::*;

        match self {
            Hz1600 => 1600.0,
            Hz800 => 800.0,
            Hz400 => 400.0,
            Hz200 => 200.0,
            Hz100 => 100.0,
            Hz50 => 50.0,
            Hz25 => 25.0,
            Hz12_5 => 12.5,
            Hz6_25 => 6.25,
            Hz3_125 => 3.125,
            Hz1_5625 => 1.5625,
        }
    }
}

impl Bitfield for AccelOdr {
    const BITMASK: u8 = 0b0000_1111;

    fn bits(self) -> u8 {
        // `ACCEL_ODR` occupies bits 3:0 in the register
        self as u8
    }
}

impl TryFrom<u8> for AccelOdr {
    type Error = SensorError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use AccelOdr::*;

        match value {
            0b0101 => Ok(Hz1600),
            0b0110 => Ok(Hz800),
            0b0111 => Ok(Hz400),
            0b1000 => Ok(Hz200),
            0b1001 => Ok(Hz100),
            0b1010 => Ok(Hz50),
            0b1011 => Ok(Hz25),
            0b1100 => Ok(Hz12_5),
            0b1101 => Ok(Hz6_25),
            0b1110 => Ok(Hz3_125),
            0b1111 => Ok(Hz1_5625),
            _ => Err(SensorError::InvalidDiscriminant),
        }
    }
}

/// Gyroscope ODR selection values
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum GyroOdr {
    /// 1.6k Hz
    Hz1600 = 0b0101,
    /// 800 Hz
    #[default]
    Hz800  = 0b0110,
    /// 400 Hz
    Hz400  = 0b0111,
    /// 200 Hz
    Hz200  = 0b1000,
    /// 100 Hz
    Hz100  = 0b1001,
    /// 50 Hz
    Hz50   = 0b1010,
    /// 25 Hz
    Hz25   = 0b1011,
    /// 12.5 Hz
    Hz12_5 = 0b1100,
}

impl GyroOdr {
    pub fn as_f32(self) -> f32 {
        use GyroOdr::*;

        match self {
            Hz1600 => 1600.0,
            Hz800 => 800.0,
            Hz400 => 400.0,
            Hz200 => 200.0,
            Hz100 => 100.0,
            Hz50 => 50.0,
            Hz25 => 25.0,
            Hz12_5 => 12.5,
        }
    }
}

impl Bitfield for GyroOdr {
    const BITMASK: u8 = 0b0000_1111;

    fn bits(self) -> u8 {
        // `GYRO_ODR` occupies bits 3:0 in the register
        self as u8
    }
}

impl TryFrom<u8> for GyroOdr {
    type Error = SensorError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use GyroOdr::*;

        match value {
            0b0101 => Ok(Hz1600),
            0b0110 => Ok(Hz800),
            0b0111 => Ok(Hz400),
            0b1000 => Ok(Hz200),
            0b1001 => Ok(Hz100),
            0b1010 => Ok(Hz50),
            0b1011 => Ok(Hz25),
            0b1100 => Ok(Hz12_5),
            _ => Err(SensorError::InvalidDiscriminant),
        }
    }
}
