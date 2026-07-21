//! Single source of truth for which pins exist on each port and which PAC PFS
//! register accessor serves each pin. `gpio.rs` and `pfsel.rs` both generate
//! their per-port modules from this table using a callback-macro pattern, so a pin
//! cannot exist in the typed API without a register mapping.
//!
//! Entry format: `(field, pin_number, arr <accessor> <index>)` for pins served
//! by a PAC register array, `(field, pin_number, reg <accessor>)` for pins
//! with a standalone register. The PAC splits each port's PmnPFS registers
//! into arrays per contiguous run of pins plus standalone registers for
//! isolated pins, and skips pins that don't exist on this device. Accessor
//! offsets were checked against the hardware manual's PmnPFS address formula
//! (0x40080800 + 0x40 * m + 0x04 * n):
//!
//!   port0: p00pfs[8] -> 0..=7,  p008pfs -> 8,  p0pfs[3] -> 13..=15
//!   port1: p10pfs[10] -> 0..=9, p1pfs[6] -> 10..=15
//!   port2: p200pfs -> 0, p201pfs -> 1, p20pfs[5] -> 5..=9, p2pfs[5] -> 10..=14
//!   port3: p300pfs -> 0, p30pfs[7] -> 1..=7
//!   port4: p40pfs[10] -> 0..=9, p4pfs[6] -> 10..=15
//!   port5: p50pfs[6] -> 0..=5
//!   port6: p60pfs[2] -> 8..=9, p610pfs -> 10
//!   port7: p708pfs -> 8
//! 
//! Hint: run from the HAL root to see the generated code for a port using the for_each_port! callback macro. P7 just
//! has a single pin, so the generated code is easier to read:
//! cargo expand --target thumbv8m.main-none-eabihf --features port7 gpio::port7   # typed pin API
//! cargo expand --target thumbv8m.main-none-eabihf --features port7 pfsel::port7  # register dispatch
macro_rules! for_each_port {
    ($callback:ident) => {
        $callback!(
            "port0", port0, Port0, Port0Pins, ra4m2_pac::Port0,
            [
                (p00, 0, arr p00pfs 0), (p01, 1, arr p00pfs 1), (p02, 2, arr p00pfs 2),
                (p03, 3, arr p00pfs 3), (p04, 4, arr p00pfs 4), (p05, 5, arr p00pfs 5),
                (p06, 6, arr p00pfs 6), (p07, 7, arr p00pfs 7),
                (p08, 8, reg p008pfs),
                (p13, 13, arr p0pfs 0), (p14, 14, arr p0pfs 1), (p15, 15, arr p0pfs 2),
            ]
        );
        $callback!(
            "port1", port1, Port1, Port1Pins, ra4m2_pac::Port1,
            [
                (p00, 0, arr p10pfs 0), (p01, 1, arr p10pfs 1), (p02, 2, arr p10pfs 2),
                (p03, 3, arr p10pfs 3), (p04, 4, arr p10pfs 4), (p05, 5, arr p10pfs 5),
                (p06, 6, arr p10pfs 6), (p07, 7, arr p10pfs 7), (p08, 8, arr p10pfs 8),
                (p09, 9, arr p10pfs 9),
                (p10, 10, arr p1pfs 0), (p11, 11, arr p1pfs 1), (p12, 12, arr p1pfs 2),
                (p13, 13, arr p1pfs 3), (p14, 14, arr p1pfs 4), (p15, 15, arr p1pfs 5),
            ]
        );
        $callback!(
            "port2", port2, Port2, Port2Pins, ra4m2_pac::Port1,
            [
                (p00, 0, reg p200pfs), (p01, 1, reg p201pfs),
                (p05, 5, arr p20pfs 0), (p06, 6, arr p20pfs 1), (p07, 7, arr p20pfs 2),
                (p08, 8, arr p20pfs 3), (p09, 9, arr p20pfs 4),
                (p10, 10, arr p2pfs 0), (p11, 11, arr p2pfs 1), (p12, 12, arr p2pfs 2),
                (p13, 13, arr p2pfs 3), (p14, 14, arr p2pfs 4),
            ]
        );
        $callback!(
            "port3", port3, Port3, Port3Pins, ra4m2_pac::Port1,
            [
                (p00, 0, reg p300pfs),
                (p01, 1, arr p30pfs 0), (p02, 2, arr p30pfs 1), (p03, 3, arr p30pfs 2),
                (p04, 4, arr p30pfs 3), (p05, 5, arr p30pfs 4), (p06, 6, arr p30pfs 5),
                (p07, 7, arr p30pfs 6),
            ]
        );
        $callback!(
            "port4", port4, Port4, Port4Pins, ra4m2_pac::Port1,
            [
                (p00, 0, arr p40pfs 0), (p01, 1, arr p40pfs 1), (p02, 2, arr p40pfs 2),
                (p03, 3, arr p40pfs 3), (p04, 4, arr p40pfs 4), (p05, 5, arr p40pfs 5),
                (p06, 6, arr p40pfs 6), (p07, 7, arr p40pfs 7), (p08, 8, arr p40pfs 8),
                (p09, 9, arr p40pfs 9),
                (p10, 10, arr p4pfs 0), (p11, 11, arr p4pfs 1), (p12, 12, arr p4pfs 2),
                (p13, 13, arr p4pfs 3), (p14, 14, arr p4pfs 4), (p15, 15, arr p4pfs 5),
            ]
        );
        $callback!(
            "port5", port5, Port5, Port5Pins, ra4m2_pac::Port0,
            [
                (p00, 0, arr p50pfs 0), (p01, 1, arr p50pfs 1), (p02, 2, arr p50pfs 2),
                (p03, 3, arr p50pfs 3), (p04, 4, arr p50pfs 4), (p05, 5, arr p50pfs 5),
            ]
        );
        $callback!(
            "port6", port6, Port6, Port6Pins, ra4m2_pac::Port0,
            [
                (p08, 8, arr p60pfs 0), (p09, 9, arr p60pfs 1), (p10, 10, reg p610pfs),
            ]
        );
        $callback!(
            "port7", port7, Port7, Port7Pins, ra4m2_pac::Port0,
            [
                (p08, 8, reg p708pfs),
            ]
        );
    };
}
pub(crate) use for_each_port;
