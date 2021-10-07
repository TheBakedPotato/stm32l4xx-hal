//! Device electronic signature
//!
//! (stored in flash memory)
//!
//! Based on the STM32F4xx HAL.

use core::str::from_utf8_unchecked;

/// This is the test voltage, in millivolts of the calibration done at the factory
pub const VDDA_CALIB_MV: u32 = 3000;

macro_rules! define_ptr_type {
    ($name: ident, $ptr: expr) => {
        impl $name {
            fn ptr() -> *const Self {
                $ptr as *const _
            }

            /// Returns a wrapped reference to the value in flash memory
            pub fn get() -> &'static Self {
                unsafe { &*Self::ptr() }
            }
        }
    };
}

/// Uniqure Device ID register
#[derive(Hash, Debug)]
#[repr(C)]
pub struct Uid {
    x: u16,
    y: u16,
    waf_lot: [u8; 8],
}
define_ptr_type!(Uid, 0x1FFF_7590);

impl Uid {
    /// X coordinate on wafer
    pub fn x(&self) -> u16 {
        self.x
    }

    /// Y coordinate on wafer
    pub fn y(&self) -> u16 {
        self.y
    }

    /// Wafer number
    pub fn waf_num(&self) -> u8 {
        self.waf_lot[0]
    }

    /// Lot number
    pub fn lot_num(&self) -> &str {
        unsafe { from_utf8_unchecked(&self.waf_lot[1..]) }
    }

    /// As a byte array
    pub fn as_bytes() -> &'static [u8; 12] {
        unsafe { &*(Self::ptr() as *const _) }
    }
}

/// Size of integrated flash
#[derive(Debug)]
#[repr(C)]
pub struct FlashSize(u16);
define_ptr_type!(FlashSize, 0x1FFF_75E0);

impl FlashSize {
    /// Read flash size in kilobytes
    pub fn kilo_bytes(&self) -> u16 {
        self.0
    }

    /// Read flash size in bytes
    pub fn bytes(&self) -> usize {
        usize::from(self.kilo_bytes()) * 1024
    }
}

/// ADC VREF calibration value is stored in at the factory
#[derive(Debug)]
#[repr(C)]
pub struct VrefCal(u16);
define_ptr_type!(VrefCal, 0x1FFF_75AA);

impl VrefCal {
    /// Read calibration value
    pub fn read(&self) -> u16 {
        self.0
    }
}

/// A temperature reading taken at 30°C stored at the factory
/// aka TS_CAL1
#[derive(Debug)]
#[repr(C)]
pub struct VtempCal30(u16);
define_ptr_type!(VtempCal30, 0x1FFF_75A8);

impl VtempCal30 {
    /// TS_CAL1_TEMP
    pub const TEMP_DEGREES: u16 = 30;
    /// Read calibration value
    pub fn read(&self) -> u16 {
        self.0
    }
}

/// A temperature reading taken at 130°C stored at the factory
/// aka TS_CAL2
#[derive(Debug)]
#[repr(C)]
pub struct VtempCal130(u16);
define_ptr_type!(VtempCal130, 0x1FFF_75CA);

impl VtempCal130 {
    /// TS_CAL2_TEMP
    /// TODO: this is 110 for L47x, 130 for L43x
    pub const TEMP_DEGREES: u16 = 130;
    /// Read calibration value
    pub fn read(&self) -> u16 {
        self.0
    }
}
