# WIP - RA4M2 Rust HAL

First attempt at a Rust HAL for the RA4M2 series microcontroller. Pretty bare bones right now:
- I2C Write
- embedded_time and half working embassy_time_driver
- Interrupt registration and clearing
- Peripheral power control
- HOCO and Main Clock oscillator control
- Some limited Port4 pin control features

# TODO:
Lots:
- I2C Read
- Some GPIO pins
- Maybe another peripheral or two

# Example
```rust
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt::info;
use ra4m2_hal::{i2c::I2c, sysc::SystemClock};

use panic_probe as _;
use defmt_rtt as _;

#[entry]
fn main() -> ! {
    let peripheral = ra4m2_pac::Peripherals::take().unwrap();

    let mut system_clk = SystemClock::<24_000_000>::new(peripheral.SYSC);
    let clock_src = system_clk.get_system_clock_src();
    let clock_freq = system_clk.get_system_clk_freq();

    info!("System Clock Source: {:?}", clock_src);
    info!("System Clock Frequency: {} Hz", clock_freq);

    let mst = peripheral.MSTP;
    let port1 = peripheral.PORT4;
    let pfs = peripheral.PFS;

    system_clk
        .set_system_clk_src(ra4m2_hal::sysc::ClockSource::MainClockOsc)
        .set_system_clk_divder(ra4m2_hal::sysc::ClockDividers::Div1)
        .set_pclkb_divider(ra4m2_hal::sysc::ClockDividers::Div1); // Each tick of the timer is at 24MHz

    ra4m2_hal::power::Power::init(mst); // Initialize power management
    ra4m2_hal::icu::Icu::init(peripheral.ICU); // Initialize ICU for interrupt handling
    ra4m2_hal::pfsel::PinFnSel::init(pfs); // Initialize Pin Function Select
    ra4m2_hal::time_driver::init(peripheral.AGT0); // Initialize the time driver with AGT0

    let clock_src = system_clk.get_system_clock_src();
    info!("Updated System Clock Source: {:?}", clock_src);

    let new_clock_freq = system_clk.get_system_clk_freq();
    info!("New System Clock Frequency: {} Hz", new_clock_freq);

    ra4m2_hal::demo::Demo::new(port1).configure_pins();

    embassy_time::Instant::now();

    let mut i2c = I2c::new(peripheral.IIC0);

    loop {
        //info!("Hello World, Renesas RA4M2 from Rust!");
        //let result = i2c.write(0x0A, &[0x01, 0x02]);
        cortex_m::asm::delay(100_000); // Adjust the delay as needed
        let seconds = embassy_time::Instant::now().as_secs();
        let millis = embassy_time::Instant::now().as_millis();
        let micros = embassy_time::Instant::now().as_micros();

        info!("Time elapsed s: {}, ms: {}, ns: {}", seconds,
            millis.checked_sub(seconds * 1000).unwrap_or(0),
            micros.checked_sub(seconds * 1_000_000).unwrap_or(0));
    }
}
```

