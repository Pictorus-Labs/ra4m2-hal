#![no_std]
#![no_main]

use core::ptr::addr_of_mut;

use cortex_m_rt::entry;
use defmt::info;
use num_traits::AsPrimitive;
use pictorus_renesas::RenesasClock;
use ra4m2_hal::{gpio::{port4::Pin, AlternateFunction, Output, PushPull}, i2c::I2c0, sysc::SystemClock};
use embedded_time::{clock::Clock, duration::{Generic, Microseconds}, fixed_point::FixedPoint, TimeInt};
use embedded_hal::digital::v2::OutputPin;

use panic_probe as _;
use defmt_rtt as _;

#[global_allocator]
static HEAP: embedded_alloc::Heap = embedded_alloc::Heap::empty();

pub fn embedded_duration_to_us<T, U>(duration: Generic<T>) -> U
where
    T: TimeInt + AsPrimitive<U> + Copy,
    U: AsPrimitive<T> + Copy,
{
    let duration_us: Microseconds<T> = duration
        .try_into()
        .expect("Could not cast generic Duration to Microseconds!");
    duration_us.integer().as_()
}

#[entry]
fn main() -> ! {
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 65_536;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(addr_of_mut!(HEAP_MEM) as usize, HEAP_SIZE) }
    }

    let mut system_clock_divider = ra4m2_hal::sysc::SystemClockDividerConfig::default();
    // Set PCLKB to 3 MHz to reduce interrupt frequency
    system_clock_divider.pckb = ra4m2_hal::sysc::ClockDividers::Div8; 

    let config = ra4m2_hal::sysc::SystemClockConfig {
        system_clock_divider,
        external_oscillator: 24_000_000, // 24 MHz
        clock_source: ra4m2_hal::sysc::ClockSource::MainClockOsc,
    };

    let (p, system_clock) = ra4m2_hal::init(config);

    let clock_src = system_clock.get_system_clock_src();
    let clock_freq = system_clock.get_system_clk_freq();

    info!("System Clock Source: {:?}", clock_src);
    info!("System Clock Frequency: {} Hz", clock_freq);

    let clock_src = system_clock.get_system_clock_src();
    info!("Updated System Clock Source: {:?}", clock_src);

    let new_clock_freq = system_clock.get_system_clk_freq();
    info!("New System Clock Frequency: {} Hz", new_clock_freq);

    let p4 = ra4m2_hal::gpio::port4::Port4::new(p.PORT4);

    let _p4_i2c_sda = p4.split().p00.into_alternate_function(ra4m2_hal::gpio::port4::PinFunction::IIC);
    let _p4_i2c_scl = p4.split().p01.into_alternate_function(ra4m2_hal::gpio::port4::PinFunction::IIC);

    let mut p4_15_blue_led = p4.split().p15.into_output_push_pull(ra4m2_hal::gpio::DriveMode::Middle);
    let mut p4_04_green_led = p4.split().p04.into_output_push_pull(ra4m2_hal::gpio::DriveMode::Middle);
    let mut p4_05_red_led = p4.split().p05.into_output_push_pull(ra4m2_hal::gpio::DriveMode::Middle);

    info!("The time is now {:?}us", embassy_time::Instant::now().as_micros());

    let clock = RenesasClock::default();

    // This demo talks to an mpu-6050 accelerometer/gyro sensor over I2C.
    let mut i2c0 = I2c0::new(p.IIC0);

    let app_start_time = clock.try_now().unwrap();

    let mut buffer = [0u8; 14]; // Fetch 14 bytes for accelerometer, temp, and gyro data

    let mut address_buffer = [0u8; 1]; // 1 byte buffer for reading address

    // Read the power buffer
    i2c0.write(0x68, &[0x6B]).unwrap();
    i2c0.read(0x68, &mut address_buffer).unwrap();
    i2c0.write(0x68, &[0x6B, 0x00]).unwrap();

    loop {
        //info!("Hello World, Renesas RA4M2 from Rust!");
        i2c0.write(0x68, &[0x3B]).unwrap();
        i2c0.read(0x68, &mut buffer).unwrap();

        let accel_x = i16::from_be_bytes([buffer[0], buffer[1]]);
        let accel_y = i16::from_be_bytes([buffer[2], buffer[3]]);
        let accel_z = i16::from_be_bytes([buffer[4], buffer[5]]);

        if accel_x < -2000 || accel_x > 2000 {
            p4_15_blue_led.set_high().unwrap();
        } else{
            p4_15_blue_led.set_low().unwrap();
        }

        if accel_y < -2000 || accel_y > 2000 {
            p4_04_green_led.set_high().unwrap();
        } else {
            p4_04_green_led.set_low().unwrap();
        }

        if accel_z < -2000 || accel_z > 2000 {
            p4_05_red_led.set_high().unwrap();
        } else {
            p4_05_red_led.set_low().unwrap();
        }


        cortex_m::asm::delay(240_000); // Adjust the delay as needed
        let us: u64 = embedded_duration_to_us(clock.try_now().unwrap() - app_start_time);
        info!("Current time: {:?}, Accel X: {}, Y: {}, Z: {}", us, accel_x, accel_y, accel_z)
    }
}