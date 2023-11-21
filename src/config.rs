use crate::{
    error::SensorError,
    register::{Bank0, Register},
};

pub(crate) trait Bitfield {
    const BITMASK: u8;
    type Reg: Register;
    const REGISTER: Self::Reg;

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
    type Reg = Bank0;
    const REGISTER: Self::Reg = Self::Reg::ACCEL_CONFIG0;

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
    type Reg = Bank0;
    const REGISTER: Self::Reg = Self::Reg::GYRO_CONFIG0;

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
    type Reg = Bank0;
    const REGISTER: Self::Reg = Self::Reg::PWR_MGMT0;

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
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AccelOdr {
    /// 1.6 kHz (LN mode)
    Hz1600   = 0b0101,
    /// 800 Hz (LN mode
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
    type Reg = Bank0;
    const REGISTER: Self::Reg = Self::Reg::ACCEL_CONFIG0;

    fn bits(self) -> u8 {
        // `ACCEL_ODR` occupies bits 3:0 in the register
        self as u8
    }
}

impl Default for AccelOdr {
    fn default() -> Self {
        Self::Hz800
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

/// Acceleration Low Power Averaging
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AccLpAvg {
    X2  = 0b000,
    X4  = 0b001,
    X8  = 0b010,
    X16 = 0b011,
    X32 = 0b100,
    X64 = 0b101,
}

impl Bitfield for AccLpAvg {
    const BITMASK: u8 = 0b0111_0000;
    type Reg = Bank0;
    const REGISTER: Self::Reg = Self::Reg::ACCEL_CONFIG1;

    fn bits(self) -> u8 {
        (self as u8) << 4
    }
}

/// Acceleration Digital Low Pass Filter options
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AccelDlpfBw {
    Bypassed = 0b000,
    Hz180    = 0b001,
    Hz121    = 0b010,
    Hz73     = 0b011,
    Hz53     = 0b100,
    Hz34     = 0b101,
    Hz25     = 0b110,
    Hz16     = 0b111,
}

impl Bitfield for AccelDlpfBw {
    const BITMASK: u8 = 0b0000_0111;
    type Reg = Bank0;
    const REGISTER: Self::Reg = Self::Reg::ACCEL_CONFIG1;

    fn bits(self) -> u8 {
        self as u8
    }
}

/// Temperature DLPF (Digital Low Pass Filter) Bandwidth
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TempDlpfBw {
    Bypassed = 0b000,
    Hz180    = 0b001,
    Hz72     = 0b010,
    Hz34     = 0b011,
    Hz16     = 0b100,
    Hz8      = 0b101,
    Hz4      = 0b110,
}
impl Bitfield for TempDlpfBw {
    const BITMASK: u8 = 0b0111_0000;
    type Reg = Bank0;
    const REGISTER: Self::Reg = Self::Reg::TEMP_CONFIG0;

    fn bits(self) -> u8 {
        (self as u8) << 4
    }
}

/// Gyroscope UI low pass filter bandwidth
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GyroLpFiltBw {
    Bypassed = 0b000,
    Hz180    = 0b001,
    Hz121    = 0b010,
    Hz73     = 0b011,
    Hz53     = 0b100,
    Hz34     = 0b101,
    Hz25     = 0b110,
    Hz16     = 0b111,
}
impl Bitfield for GyroLpFiltBw {
    const BITMASK: u8 = 0b0000_0111;
    type Reg = Bank0;
    const REGISTER: Self::Reg = Self::Reg::GYRO_CONFIG1;

    fn bits(self) -> u8 {
        self as u8
    }
}
/// Gyroscope ODR selection values
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GyroOdr {
    /// 1.6k Hz
    Hz1600 = 0b0101,
    /// 800 Hz
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
    type Reg = Bank0;
    const REGISTER: Self::Reg = Self::Reg::GYRO_CONFIG0;

    fn bits(self) -> u8 {
        // `GYRO_ODR` occupies bits 3:0 in the register
        self as u8
    }
}

impl Default for GyroOdr {
    fn default() -> Self {
        Self::Hz800
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum SoftReset {
    Enabled   = 0b0,
    _Disabled = 0b1,
}

impl Bitfield for SoftReset {
    const BITMASK: u8 = 0b0001_0000;
    type Reg = Bank0;
    const REGISTER: Self::Reg = Self::Reg::SIGNAL_PATH_RESET;

    fn bits(self) -> u8 {
        (self as u8) << 4
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum MClkReady {
    Running    = 0b0,
    NotRunning = 0b1,
}

impl Bitfield for MClkReady {
    const BITMASK: u8 = 0b0000_1000;
    type Reg = Bank0;
    const REGISTER: Self::Reg = Self::Reg::MCLK_RDY;

    fn bits(self) -> u8 {
        (self as u8) << 3
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum SpiWireCount {
    ThreeWire = 0b0,
    FourWire  = 0b1,
}

impl Bitfield for SpiWireCount {
    const BITMASK: u8 = 0b0000_0100;
    type Reg = Bank0;
    const REGISTER: Self::Reg = Self::Reg::DEVICE_CONFIG;

    fn bits(self) -> u8 {
        (self as u8) << 2
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum SpiMode {
    Mode0And3 = 0b0,
    Mode1And2 = 0b1,
}

impl Bitfield for SpiMode {
    const BITMASK: u8 = 0b0000_0001;
    type Reg = Bank0;
    const REGISTER: Self::Reg = Self::Reg::DEVICE_CONFIG;

    fn bits(self) -> u8 {
        self as u8
    }
}

/// Controls slew rate for output pin 14 when device is in I3CSM DDR protocol.
/// While in I3CSM operation, the device automatically switches to use
/// I3C_DDR_SLEW_RATE after receiving ENTHDR0 ccc command from the host.
/// The device automatically switches back to I3C_SDR_SLEW_RATE after the
/// host issues HDR_EXIT pattern.
///
/// This register field should not be programmed in I3C/DDR mode.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum I3CDdrSlewRate {
    /// Min 20ns; Typ 40ns; Max 60ns
    M20T40M60 = 0b000,
    /// Min 12ns; Typ 24ns; Max 36ns
    M12T24M36 = 0b001,
    /// Min 6ns; Typ 12ns; Max 19ns
    M6T12M19  = 0b010,
    /// Min 4ns; Typ 8ns; Max 14ns
    M4T8M14   = 0b011,
    /// Min 2ns; Typ 4ns; Max 8ns
    M2T4M8    = 0b100,
    /// Max 2ns
    M2        = 0b101,
}

impl Bitfield for I3CDdrSlewRate {
    const BITMASK: u8 = 0b0011_1000;
    type Reg = Bank0;
    const REGISTER: Self::Reg = Self::Reg::DRIVE_CONFIG1;

    fn bits(self) -> u8 {
        (self as u8) << 3
    }
}

/// Controls slew rate for output pin 14 in I3CSM SDR protocol.
/// After device reset, I2C_SLEW_RATE is used by default. If I3CSM feature is
/// enabled, the device automatically switches to use I3C_SDR_SLEW_RATE
/// after receiving 0x7E+W message (an I3CSM broadcast message).
///
/// This register field should not be programmed in I3C/DDR mode.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum I3CSdrSlewRate {
    /// Min 20ns; Typ 40ns; Max 60ns
    M20T40M60 = 0b000,
    /// Min 12ns; Typ 24ns; Max 36ns
    M12T24M36 = 0b001,
    /// Min 6ns; Typ 12ns; Max 19ns
    M6T12M19  = 0b010,
    /// Min 4ns; Typ 8ns; Max 14ns
    M4T8M14   = 0b011,
    /// Min 2ns; Typ 4ns; Max 8ns
    M2T4M8    = 0b100,
    /// Max 2ns
    M2        = 0b101,
}

impl Bitfield for I3CSdrSlewRate {
    const BITMASK: u8 = 0b0000_0111;
    type Reg = Bank0;
    const REGISTER: Self::Reg = Self::Reg::DRIVE_CONFIG1;

    fn bits(self) -> u8 {
        self as u8
    }
}

/// Controls slew rate for output pin 14 in I2C mode.
/// After device reset, the I2C_SLEW_RATE is used by default. If the 1st write
/// operation from host is an SPI transaction, the device automatically switches
/// to SPI_SLEW_RATE. If I3CSM feature is enabled, the device automatically
/// switches to I3C_SDR_SLEW_RATE after receiving 0x7E+W message (an I3C
/// broadcast message).
///
/// This register field should not be programmed in I3C/DDR mode.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum I2CSlewRate {
    /// Min 20ns; Typ 40ns; Max 60ns
    M20T40M60 = 0b000,
    /// Min 12ns; Typ 24ns; Max 36ns
    M12T24M36 = 0b001,
    /// Min 6ns; Typ 12ns; Max 19ns
    M6T12M19  = 0b010,
    /// Min 4ns; Typ 8ns; Max 14ns
    M4T8M14   = 0b011,
    /// Min 2ns; Typ 4ns; Max 8ns
    M2T4M8    = 0b100,
    /// Max 2ns
    M2        = 0b101,
}

impl Bitfield for I2CSlewRate {
    const BITMASK: u8 = 0b0011_1000;
    type Reg = Bank0;
    const REGISTER: Self::Reg = Self::Reg::DRIVE_CONFIG2;

    fn bits(self) -> u8 {
        (self as u8) << 3
    }
}

/// Configure drive strength for all output pins in all modes (SPI3, SPI4, I2C,
/// I3CSM) excluding pin 14.
///
/// This register field should not be programmed in I3C/DDR mode.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum AllSlewRate {
    /// Min 20ns; Typ 40ns; Max 60ns
    M20T40M60 = 0b000,
    /// Min 12ns; Typ 24ns; Max 36ns
    M12T24M36 = 0b001,
    /// Min 6ns; Typ 12ns; Max 19ns
    M6T12M19  = 0b010,
    /// Min 4ns; Typ 8ns; Max 14ns
    M4T8M14   = 0b011,
    /// Min 2ns; Typ 4ns; Max 8ns
    M2T4M8    = 0b100,
    /// Max 2ns
    M2        = 0b101,
}

impl Bitfield for AllSlewRate {
    const BITMASK: u8 = 0b0000_0111;
    type Reg = Bank0;
    const REGISTER: Self::Reg = Self::Reg::DRIVE_CONFIG2;

    fn bits(self) -> u8 {
        self as u8
    }
}

/// Controls slew rate for output pin 14 in SPI 3-wire mode. In SPI 4-wire mode
/// this register controls the slew rate of pin 1 as it is used as an output in
/// SPI 4- wire mode only. After chip reset, the I2C_SLEW_RATE is used by
/// default for pin 14 pin. If the 1st write operation from the host is an
/// SPI3/4 transaction, the device automatically switches to SPI_SLEW_RATE.
///
/// This register field should not be programmed in I3C/DDR mode.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum SpiSlewRate {
    /// Min 20ns; Typ 40ns; Max 60ns
    M20T40M60 = 0b000,
    /// Min 12ns; Typ 24ns; Max 36ns
    M12T24M36 = 0b001,
    /// Min 6ns; Typ 12ns; Max 19ns
    M6T12M19  = 0b010,
    /// Min 4ns; Typ 8ns; Max 14ns
    M4T8M14   = 0b011,
    /// Min 2ns; Typ 4ns; Max 8ns
    M2T4M8    = 0b100,
    /// Max 2ns
    M2        = 0b101,
}

impl Bitfield for SpiSlewRate {
    const BITMASK: u8 = 0b0000_0111;
    type Reg = Bank0;
    const REGISTER: Self::Reg = Self::Reg::DRIVE_CONFIG2;

    fn bits(self) -> u8 {
        self as u8
    }
}
