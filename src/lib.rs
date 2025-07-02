#![no_std]

use ra4m2_pac::Peripherals;

use crate::sysc::SystemClock;

pub mod sysc;
pub mod gpio;
pub mod i2c;
pub mod power;
pub mod time_driver;
pub mod icu;
pub mod pfsel;

mod sealed {
    pub trait Sealed {}
}

pub fn init(config: sysc::SystemClockConfig) -> (Peripherals, SystemClock) {
    let peripheral = ra4m2_pac::Peripherals::take().expect("msg: Failed to take RA4M2 peripherals");

    let mut system_clk = SystemClock::new(peripheral.SYSC, config);

    system_clk
        .set_system_clk_src(config.clock_source)
        .set_system_clk_divder(config.system_clock_divider.ick)
        .set_pclkb_divider(config.system_clock_divider.pckb); // Each tick of the timer is at 24MHz

    crate::power::Power::init(peripheral.MSTP); // Initialize power management
    crate::icu::Icu::init(peripheral.ICU); // Initialize ICU for interrupt handling
    crate::pfsel::PinFnSel::init(peripheral.PFS); // Initialize Pin Function Select

    #[cfg(feature = "agt0")]
    crate::time_driver::init(peripheral.AGT0, 3_000_000); // Initialize the time driver with AGT0

    (peripheral, system_clk)
}