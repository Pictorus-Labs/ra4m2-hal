use core::cell::RefCell;

use ra4m2_pac::{mstp::{mstpcrb::Mstpb9, mstpcrd::Mstpd3}, Mstp};

static POWER: cortex_m::interrupt::Mutex<RefCell<Option<Mstp>>> = cortex_m::interrupt::Mutex::new(RefCell::new(None));

/// Takes control of the power management system.
pub struct Power {}

impl Power {
    pub fn init(mstp: Mstp) {
        cortex_m::interrupt::free(|cs| {
            POWER.borrow(cs).replace(Some(mstp));
        });
    }
}

/// Enables the power management system for the I2C0 module
pub fn enable_i2c0(cs: &cortex_m::interrupt::CriticalSection) {
    // Enable I2C0 module
    unsafe {
        if let Some(mstp) = POWER.borrow(cs).borrow_mut().as_mut() {
            mstp.mstpcrb().modify(|w| w.mstpb9().set(Mstpb9::_0)); // Set the bit to 0 to enable
        }
    }
}

/// Enables the power management system for the AGT0 module
pub fn enable_agt0(cs: &cortex_m::interrupt::CriticalSection) {
    // Enable AGT0 module
    unsafe {
        if let Some(mstp) = POWER.borrow(cs).borrow_mut().as_mut() {
            mstp.mstpcrd().modify(|w| w.mstpd3().set(Mstpd3::_0)); // Set the bit to 0 to enable
        }
    }
}

