use core::cell::RefCell;

use crate::gpio4::{PinFunctionPort4};

static PFSEL: cortex_m::interrupt::Mutex<RefCell<Option<ra4m2_pac::Pfs>>> = cortex_m::interrupt::Mutex::new(RefCell::new(None));

/// Pin Function Select (PFS) control structure
pub struct PinFnSel {}

impl PinFnSel {
    pub fn init(pfs: ra4m2_pac::Pfs) -> Self {
        cortex_m::interrupt::free(|cs| {
            PFSEL.borrow(cs).replace(Some(pfs));
        });
        PinFnSel {}
    }
}

/// Disable write protection for PFS registers
pub fn _disable_write_protect(cs: &cortex_m::interrupt::CriticalSection) {
    if let Some(pfs) = PFSEL.borrow(cs).borrow_mut().as_mut() {
        unsafe {
            pfs.pwpr().modify(|w| w.b0wi().set(ra4m2_pac::pfs::pwpr::B0Wi::_0));
            pfs.pwpr().modify(|w| w.pfswe().set(ra4m2_pac::pfs::pwpr::Pfswe::_1));
        }
    }
}

/// Enable write protection for PFS registers
pub fn _enable_write_protect(cs: &cortex_m::interrupt::CriticalSection) {
    if let Some(pfs) = PFSEL.borrow(cs).borrow_mut().as_mut() {
        unsafe {
            pfs.pwpr().modify(|w| w.pfswe().set(ra4m2_pac::pfs::pwpr::Pfswe::_0));
            pfs.pwpr().modify(|w| w.b0wi().set(ra4m2_pac::pfs::pwpr::B0Wi::_1));
        }
    }
}

/// Clunky first pass implementation to set pin function for Port 4.
pub fn set_pin_function_port4(pin: u8, function: PinFunctionPort4, open_drain: bool, pull_up_control: bool, port_mode: bool) {
    cortex_m::interrupt::free(|cs| {
        _disable_write_protect(cs);

        let odrain = if open_drain {
            ra4m2_pac::pfs::p40pfs::Ncodr::_1
        } else {
            ra4m2_pac::pfs::p40pfs::Ncodr::_0
        };

        let puc = if pull_up_control {
            ra4m2_pac::pfs::p40pfs::Pcr::_1
        }else{
            ra4m2_pac::pfs::p40pfs::Pcr::_0
        };

        let pmc = if port_mode {
            ra4m2_pac::pfs::p40pfs::Pmr::_1
        } else {
            ra4m2_pac::pfs::p40pfs::Pmr::_0
        };

        if let Some(pfs) = PFSEL.borrow(cs).borrow_mut().as_mut() {
            unsafe {
                pfs.p40pfs().get(pin as usize).modify(|w| {
                    w.psel().set(function.into())
                        .pmr().set(pmc)
                        .ncodr().set(odrain)
                        .pcr().set(puc)
                });
            }
        }

        _enable_write_protect(cs);
    });
}

