#![no_std]

use crate::sysc::SystemClock;

pub mod sysc;
pub mod gpio4;
pub mod i2c;
pub mod power;
pub mod time_driver;
pub mod icu;
pub mod pfsel;
pub mod demo;


pub fn init(config: sysc::SystemClockConfig) -> SystemClock {
    let peripheral = ra4m2_pac::Peripherals::take().unwrap();

    let mut system_clk = SystemClock::new(peripheral.SYSC, config);

    system_clk
        .set_system_clk_src(config.clock_source)
        .set_system_clk_divder(config.system_clock_divider.ick)
        .set_pclkb_divider(config.system_clock_divider.pckb); // Each tick of the timer is at 24MHz

    crate::power::Power::init(peripheral.MSTP); // Initialize power management
    crate::icu::Icu::init(peripheral.ICU); // Initialize ICU for interrupt handling
    crate::pfsel::PinFnSel::init(peripheral.PFS); // Initialize Pin Function Select
    crate::time_driver::init(peripheral.AGT0, 3_000_000); // Initialize the time driver with AGT0

    system_clk
}