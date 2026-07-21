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

// The per-port modules below are generated from the shared pin table in
// `port_map.rs` — one entry per existing pin, carrying the PAC accessor that
// serves it. See that file for the accessor/offset notes. All of the
// generated PmnPFS register types have an identical field set, so the same
// macro bodies type-check for every accessor.

/// Resolves a pin-table accessor entry to its PmnPFS register reference.
macro_rules! pfs_reg {
    ($pfs:ident, arr $acc:ident $idx:literal) => {
        $pfs.$acc().get($idx)
    };
    ($pfs:ident, reg $acc:ident) => {
        $pfs.$acc()
    };
}

/// Writes the full pin configuration to one PmnPFS register.
macro_rules! pfs_write_config {
    (
        $reg:expr, $direction:ident, $pull_up_control:ident, $open_drain:ident, $drive:ident,
        $event:ident, $interrupt_enable:ident, $analog_select:ident, $function:ident, $port_mode:ident
    ) => {
        $reg.modify(|w| {
            w.psel().set($function as u8)
                .pdr().set(($direction as u8).into())
                .pcr().set(($pull_up_control as u8).into())
                .ncodr().set(($open_drain as u8).into())
                .dscr().set(($drive as u8).into())
                .eofr().set(($event as u8).into())
                .isel().set(($interrupt_enable as u8).into())
                .pmr().set(($port_mode as u8).into())
                .asel().set(($analog_select as u8).into())
        })
    };
}

/// Generates the PFS access module for one port. Invoked for every port via
/// `for_each_port!` in `port_map.rs`; the port/pins struct names and PAC type
/// in the table are consumed by the `gpio_port!` callback and ignored here.
macro_rules! pfs_port {
    (
        $feature:literal, $mod_name:ident, $_port_struct:ident, $_pins_struct:ident, $_pac_ty:ty,
        [ $( ($_field:ident, $n:literal, $($acc:tt)+) ),+ $(,)? ]
    ) => {
        #[cfg(feature = $feature)]
        pub mod $mod_name {
            use crate::gpio::{
                AnalogInput, DrainControl, DriveMode, InterruptEnable, InterruptEvent, OutputValue,
                PinFunction, PortDirection, PortMode, PullUpMode,
            };
            use crate::pfsel::{_disable_write_protect, _enable_write_protect, PFSEL};

            pub fn set_pin_function(
                pin: u8,
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
                            match pin {
                                $(
                                    $n => pfs_write_config!(
                                        pfs_reg!(pfs, $($acc)+),
                                        direction, pull_up_control, open_drain, drive,
                                        event, interrupt_enable, analog_select, function, port_mode
                                    ),
                                )+
                                // Pins that don't exist on this port; unreachable through
                                // the typed Pin API.
                                _ => {}
                            }
                        }
                    }

                    _enable_write_protect(cs);
                });

                // Flush the buffered configuration write so the pin (or the
                // peripheral now muxed onto it) is actually configured before
                // the caller touches it.
                cortex_m::asm::dsb();
            }

            pub fn set_pin_value(pin: u8, output: OutputValue) {
                cortex_m::interrupt::free(|cs| {
                    _disable_write_protect(cs);

                    if let Some(pfs) = PFSEL.borrow(cs).borrow_mut().as_mut() {
                        unsafe {
                            match pin {
                                $(
                                    $n => pfs_reg!(pfs, $($acc)+)
                                        .modify(|w| w.podr().set((output as u8).into())),
                                )+
                                _ => {}
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
                            match pin {
                                $(
                                    $n => pfs_reg!(pfs, $($acc)+).read().pidr().get().0 == 1,
                                )+
                                _ => false,
                            }
                        }
                    } else {
                        false
                    }
                })
            }
        }
    };
}

use crate::port_map::for_each_port;

for_each_port!(pfs_port);
