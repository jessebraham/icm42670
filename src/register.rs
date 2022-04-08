#![allow(non_camel_case_types)]
#![allow(unused)]

// All reads and writes actually go through User Bank 0, and the remaining banks
// are accessed via this bank; as such, User Bank 0 has been omitted, given that
// we are not actually able to "select" it.
#[derive(Debug, Clone, Copy)]
pub(crate) enum RegisterBank {
    MReg1,
    MReg2,
    MReg3,
}

impl RegisterBank {
    /// The block selection value for a given register bank
    pub fn blk_sel(self) -> u8 {
        match self {
            RegisterBank::MReg1 => 0x00,
            RegisterBank::MReg2 => 0x28,
            RegisterBank::MReg3 => 0x50,
        }
    }
}

pub(crate) trait Register {
    /// Get the address of the register
    fn addr(&self) -> u8;

    /// Is the register read-only?
    fn read_only(&self) -> bool;
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Bank0 {
    MCLK_RDY          = 0x00,
    DEVICE_CONFIG     = 0x01,
    SIGNAL_PATH_RESET = 0x02,
    DRIVE_CONFIG1     = 0x03,
    DRIVE_CONFIG2     = 0x04,
    DRIVE_CONFIG3     = 0x05,
    INT_CONFIG        = 0x06,
    TEMP_DATA1        = 0x09,
    TEMP_DATA0        = 0x0A,
    ACCEL_DATA_X1     = 0x0B,
    ACCEL_DATA_X0     = 0x0C,
    ACCEL_DATA_Y1     = 0x0D,
    ACCEL_DATA_Y0     = 0x0E,
    ACCEL_DATA_Z1     = 0x0F,
    ACCEL_DATA_Z0     = 0x10,
    GYRO_DATA_X1      = 0x11,
    GYRO_DATA_X0      = 0x12,
    GYRO_DATA_Y1      = 0x13,
    GYRO_DATA_Y0      = 0x14,
    GYRO_DATA_Z1      = 0x15,
    GYRO_DATA_Z0      = 0x16,
    TMST_FSYNCH       = 0x17,
    TMST_FSYNCL       = 0x18,
    APEX_DATA4        = 0x1D,
    APEX_DATA5        = 0x1E,
    PWR_MGMT0         = 0x1F,
    GYRO_CONFIG0      = 0x20,
    ACCEL_CONFIG0     = 0x21,
    TEMP_CONFIG0      = 0x22,
    GYRO_CONFIG1      = 0x23,
    ACCEL_CONFIG1     = 0x24,
    APEX_CONFIG0      = 0x25,
    APEX_CONFIG1      = 0x26,
    WOM_CONFIG        = 0x27,
    FIFO_CONFIG1      = 0x28,
    FIFO_CONFIG2      = 0x29,
    FIFO_CONFIG3      = 0x2A,
    INT_SOURCE0       = 0x2B,
    INT_SOURCE1       = 0x2C,
    INT_SOURCE3       = 0x2D,
    INT_SOURCE4       = 0x2E,
    FIFO_LOST_PKT0    = 0x2F,
    FIFO_LOST_PKT1    = 0x30,
    APEX_DATA0        = 0x31,
    APEX_DATA1        = 0x32,
    APEX_DATA2        = 0x33,
    APEX_DATA3        = 0x34,
    INTF_CONFIG0      = 0x35,
    INTF_CONFIG1      = 0x36,
    INT_STATUS_DRDY   = 0x39,
    INT_STATUS        = 0x3A,
    INT_STATUS2       = 0x3B,
    INT_STATUS3       = 0x3C,
    FIFO_COUNTH       = 0x3D,
    FIFO_COUNTL       = 0x3E,
    FIFO_DATA         = 0x3F,
    WHO_AM_I          = 0x75,
    BLK_SEL_W         = 0x79,
    MADDR_W           = 0x7A,
    M_W               = 0x7B,
    BLK_SEL_R         = 0x7C,
    MADDR_R           = 0x7D,
    M_R               = 0x7E,
}

impl Register for Bank0 {
    fn addr(&self) -> u8 {
        *self as u8
    }

    fn read_only(&self) -> bool {
        use Bank0::*;

        matches!(
            self,
            MCLK_RDY
                | TEMP_DATA1
                | TEMP_DATA0
                | ACCEL_DATA_X1
                | ACCEL_DATA_X0
                | ACCEL_DATA_Y1
                | ACCEL_DATA_Y0
                | ACCEL_DATA_Z1
                | ACCEL_DATA_Z0
                | GYRO_DATA_X1
                | GYRO_DATA_X0
                | GYRO_DATA_Y1
                | GYRO_DATA_Y0
                | GYRO_DATA_Z1
                | GYRO_DATA_Z0
                | TMST_FSYNCH
                | TMST_FSYNCL
                | APEX_DATA4
                | APEX_DATA5
                | FIFO_LOST_PKT0
                | FIFO_LOST_PKT1
                | APEX_DATA0
                | APEX_DATA1
                | APEX_DATA2
                | APEX_DATA3
                | FIFO_COUNTH
                | FIFO_COUNTL
                | FIFO_DATA
                | WHO_AM_I
        )
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy)]
pub(crate) enum Mreg1 {
    TMST_CONFIG1    = 0x00,
    FIFO_CONFIG5    = 0x01,
    FIFO_CONFIG6    = 0x02,
    FSYNC_CONFIG    = 0x03,
    INT_CONFIG0     = 0x04,
    INT_CONFIG1     = 0x05,
    SENSOR_CONFIG3  = 0x06,
    ST_CONFIG       = 0x13,
    SELFTEST        = 0x14,
    INTF_CONFIG6    = 0x23,
    INTF_CONFIG10   = 0x25,
    INTF_CONFIG7    = 0x28,
    OTP_CONFIG      = 0x2B,
    INT_SOURCE6     = 0x2F,
    INT_SOURCE7     = 0x30,
    INT_SOURCE8     = 0x31,
    INT_SOURCE9     = 0x32,
    INT_SOURCE10    = 0x33,
    APEX_CONFIG2    = 0x44,
    APEX_CONFIG3    = 0x45,
    APEX_CONFIG4    = 0x46,
    APEX_CONFIG5    = 0x47,
    APEX_CONFIG9    = 0x48,
    APEX_CONFIG10   = 0x49,
    APEX_CONFIG11   = 0x4A,
    ACCEL_WOM_X_THR = 0x4B,
    ACCEL_WOM_Y_THR = 0x4C,
    ACCEL_WOM_Z_THR = 0x4D,
    OFFSET_USER0    = 0x4E,
    OFFSET_USER1    = 0x4F,
    OFFSET_USER2    = 0x50,
    OFFSET_USER3    = 0x51,
    OFFSET_USER4    = 0x52,
    OFFSET_USER5    = 0x53,
    OFFSET_USER6    = 0x54,
    OFFSET_USER7    = 0x55,
    OFFSET_USER8    = 0x56,
    ST_STATUS1      = 0x63,
    ST_STATUS2      = 0x64,
    FDR_CONFIG      = 0x66,
    APEX_CONFIG12   = 0x67,
}

impl Register for Mreg1 {
    fn addr(&self) -> u8 {
        *self as u8
    }

    fn read_only(&self) -> bool {
        matches!(self, Mreg1::ST_STATUS1 | Mreg1::ST_STATUS2)
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Mreg2 {
    OTP_CTRL7 = 0x06,
}

impl Register for Mreg2 {
    fn addr(&self) -> u8 {
        *self as u8
    }

    fn read_only(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Mreg3 {
    XA_ST_DATA = 0x00,
    YA_ST_DATA = 0x01,
    ZA_ST_DATA = 0x02,
    XG_ST_DATA = 0x03,
    YG_ST_DATA = 0x04,
    ZG_ST_DATA = 0x05,
}

impl Register for Mreg3 {
    fn addr(&self) -> u8 {
        *self as u8
    }

    fn read_only(&self) -> bool {
        true
    }
}
