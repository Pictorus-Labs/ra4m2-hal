#![no_std]
#![no_main]

extern crate alloc;

use alloc::{boxed::Box, vec::Vec};
use core::convert::Infallible;
use core::ptr::addr_of_mut;

use cortex_m_rt::entry;
use embedded_alloc::LlffHeap as Heap;
use embedded_hal::digital::OutputPin;
use log::info;

use panic_probe as _;

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[entry]
fn main() -> ! {
    rtt_target::rtt_init_log!();

    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 4096;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(addr_of_mut!(HEAP_MEM) as usize, HEAP_SIZE) }
    }

    let config = ra4m2_hal::sysc::SystemClockConfig {
        system_clock_divider: ra4m2_hal::sysc::SystemClockDividerConfig::default(),
        external_oscillator: 24_000_000, // 24 MHz
        clock_source: ra4m2_hal::sysc::ClockSource::MainClockOsc,
    };

    let (p, system_clock) = ra4m2_hal::init(config);

    let clock_freq = system_clock.get_system_clk_freq();
    info!("System Clock Frequency: {} Hz", clock_freq);

    use ra4m2_hal::gpio::{port0, port1, port2, port3, port4, port5, port6, port7, DriveMode};

    let p0 = port0::Port0::new(p.PORT0).split();
    let p1 = port1::Port1::new(p.PORT1).split();
    let p2 = port2::Port2::new(p.PORT2).split();
    let p3 = port3::Port3::new(p.PORT3).split();
    let p4 = port4::Port4::new(p.PORT4).split();
    let p5 = port5::Port5::new(p.PORT5).split();
    let p6 = port6::Port6::new(p.PORT6).split();
    let p7 = port7::Port7::new(p.PORT7).split();

    // Pins deliberately left alone:
    //   P108 (SWDIO), P109 (TDO/SWO), P110 (TDI), P300 (SWCLK) - debug port
    //   P200 (NMI, input-only) and P201 (MD, boot mode)
    //   P212/P213 - main clock oscillator, in use with the MainClockOsc config above
    let mut pins: Vec<Box<dyn OutputPin<Error = Infallible>>> = Vec::new();

    pins.push(Box::new(p0.p05.into_output_push_pull(DriveMode::Middle))); // P005
    pins.push(Box::new(p0.p06.into_output_push_pull(DriveMode::Middle))); // P006

    pins.push(Box::new(p0.p00.into_output_push_pull(DriveMode::Middle))); // P000
    pins.push(Box::new(p0.p01.into_output_push_pull(DriveMode::Middle))); // P001
    pins.push(Box::new(p0.p02.into_output_push_pull(DriveMode::Middle))); // P002
    pins.push(Box::new(p0.p03.into_output_push_pull(DriveMode::Middle))); // P003

    pins.push(Box::new(p1.p05.into_output_push_pull(DriveMode::Middle))); // P105
    pins.push(Box::new(p1.p06.into_output_push_pull(DriveMode::Middle))); // P106
    pins.push(Box::new(p1.p11.into_output_push_pull(DriveMode::Middle))); // P111
    pins.push(Box::new(p1.p12.into_output_push_pull(DriveMode::Middle))); // P112

    pins.push(Box::new(p2.p05.into_output_push_pull(DriveMode::Middle))); // P205
    pins.push(Box::new(p2.p06.into_output_push_pull(DriveMode::Middle))); // P206

    pins.push(Box::new(p3.p02.into_output_push_pull(DriveMode::Middle))); // P302
    pins.push(Box::new(p3.p03.into_output_push_pull(DriveMode::Middle))); // P303

    // The three user LEDs on the EK-RA4M2, so the toggling is visible without
    // a scope or logic analyzer.
    pins.push(Box::new(p4.p04.into_output_push_pull(DriveMode::Middle))); // P404, green LED
    pins.push(Box::new(p4.p05.into_output_push_pull(DriveMode::Middle))); // P405, red LED
    pins.push(Box::new(p4.p15.into_output_push_pull(DriveMode::Middle))); // P415, blue LED

    pins.push(Box::new(p5.p00.into_output_push_pull(DriveMode::Middle))); // P500
    pins.push(Box::new(p5.p01.into_output_push_pull(DriveMode::Middle))); // P501

    pins.push(Box::new(p6.p08.into_output_push_pull(DriveMode::Middle))); // P608
    pins.push(Box::new(p7.p08.into_output_push_pull(DriveMode::Middle))); // P708

    info!("Toggling {} pins", pins.len());

    let mut high = false;
    loop {
        high = !high;

        for pin in pins.iter_mut() {
            if high {
                pin.set_high().unwrap();
            } else {
                pin.set_low().unwrap();
            }
        }

        info!("Pins set {}", if high { "high" } else { "low" });

        // System clock is 24 MHz / Div4 = 6 MHz, so ~0.5s per toggle
        cortex_m::asm::delay(3_000_000);
    }
}
