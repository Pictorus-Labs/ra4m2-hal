use core::cell::RefCell;

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

pub mod port4 {
    use crate::{gpio::{port4::PinFunction, AnalogInput, DrainControl, DriveMode, InterruptEnable, InterruptEvent, OutputValue, PortDirection, PortMode, PullUpMode}, pfsel::{_disable_write_protect, _enable_write_protect, PFSEL}};

    // The SVD 2 Rust file does something that is pretty unexpected. the Port Function Select (PFS)
    // registers provide the most control for configuring pins, and the data sheet describes them as
    // grouped by Port M (port number, 0 - 7) and the N (pin number, 0 to Port Pin Number).
    // 
    // In SVD 2 Rust, these are grouped into a vector of p40pfs and p4pfs registers for Port 4. p40pfs
    // has a dimension of 10 and p4pfs has a dimension of 6, representing the 16 pins of Port 4.
    // Sanity check the address of the registers for the PFS against the Datasheet when implementing new
    // ports to make sure the addresses are correct and match.
    //
    // No idea why it is split this way. 

    const P40_DIMENSION: u8 = 10; // P40PFS has 10 elements
    const _P4_DIMENSION: u8 = 6;   // P4PFS has

    /// Clunky first pass implementation to set pin function for Port 4.
    pub fn set_pin_function(pin: u8, 
        direction: PortDirection,
        pull_up_control: PullUpMode, 
        open_drain: DrainControl, 
        drive: DriveMode,
        event: InterruptEvent,
        interrupt_enable: InterruptEnable,
        analog_select: AnalogInput,
        function: PinFunction, 
        port_mode: PortMode,
    ) {
        cortex_m::interrupt::free(|cs| {
            _disable_write_protect(cs);

            if let Some(pfs) = PFSEL.borrow(cs).borrow_mut().as_mut() {
                unsafe {
                    // Get the correct PFS register from SVD2Rust
                    if pin < P40_DIMENSION {
                        pfs.p40pfs().get(pin as usize).modify(|w| {
                            w.psel().set(function as u8)
                            .pdr().set((direction as u8).into())
                            .pcr().set((pull_up_control as u8).into())
                            .ncodr().set((open_drain as u8).into())
                            .dscr().set((drive as u8).into())
                            .eofr().set((event as u8).into())
                            .isel().set((interrupt_enable as u8).into())
                            .pmr().set((port_mode as u8).into())
                            .asel().set((analog_select as u8).into())
                            .psel().set(function as u8)
                        });
                    }else{
                        pfs.p4pfs().get((pin - P40_DIMENSION) as usize).modify(|w| {
                            w.psel().set(function as u8)
                            .pdr().set((direction as u8).into())
                            .pcr().set((pull_up_control as u8).into())
                            .ncodr().set((open_drain as u8).into())
                            .dscr().set((drive as u8).into())
                            .eofr().set((event as u8).into())
                            .isel().set((interrupt_enable as u8).into())
                            .pmr().set((port_mode as u8).into())
                            .asel().set((analog_select as u8).into())
                            .psel().set(function as u8)
                        });
                    }
                }
            }

            _enable_write_protect(cs);
        });
    }

    pub fn set_pin_value(pin: u8, output: OutputValue) {
        cortex_m::interrupt::free(|cs| {
            _disable_write_protect(cs);

            if let Some(pfs) = PFSEL.borrow(cs).borrow_mut().as_mut() {
                unsafe {
                    if pin < P40_DIMENSION {
                        pfs.p40pfs().get(pin as usize).modify(|w| w.podr().set((output as u8).into()));
                    } else {
                        pfs.p4pfs().get((pin - P40_DIMENSION) as usize).modify(|w| w.podr().set((output as u8).into()));
                    }
                }
            }

            _enable_write_protect(cs);
        });
    }

    pub fn get_pin_value(pin: u8) -> bool {
        cortex_m::interrupt::free(|cs| {
            if let Some(pfs) = PFSEL.borrow(cs).borrow_mut().as_mut() {
                unsafe {
                    if pin < P40_DIMENSION {
                        pfs.p40pfs().get(pin as usize).read().pidr().get().0 == 1
                    } else {
                        pfs.p4pfs().get((pin - P40_DIMENSION) as usize).read().pidr().get().0 == 1
                    }
                }
            } else {
                false
            }
        })
    }
}

