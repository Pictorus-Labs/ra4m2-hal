use embassy_time_driver::Driver;
use embedded_time::{Clock, Instant};
use ra4m2_pac::{agt0::agtcr::Tstart, NoBitfieldReg};
use core::{cell::RefCell, sync::atomic::{AtomicU32, Ordering}};
use ra4m2_pac::{interrupt};
use crate::{icu::{clear_interrupt, register_interrupt}, power};

/// AGT0 is a 16-bit timer
#[cfg(feature = "agt0")]
const OVERFLOW_COUNT: u16 = u16::max_value();

#[cfg(feature = "agt0")]
type T = ra4m2_pac::Agt0;

#[cfg(feature = "agt0")]
const UNDERFLOW_EVENT: u16 = 0x040; 

#[cfg(feature = "agt0")]
const _AGT0CCRA_EVENT: u16 = 0x041; // AGT0 Compare Match A event, TODO: use for alarms

static TIMER: cortex_m::interrupt::Mutex<RefCell<Option<T>>> = cortex_m::interrupt::Mutex::new(RefCell::new(None));

static TIMER_CLOCK_FREQ: AtomicU32 = AtomicU32::new(3_000_000); // 3MHz, each tick is 1/3 of a millisecond

#[cfg(feature = "agt0")]
#[interrupt]
fn IEL95() {
    clear_interrupt(interrupt::IEL95);
    let current = DRIVER.period.load(Ordering::Relaxed);
    DRIVER.period.store(current + 1, Ordering::Relaxed);
}

/// RenesasClock is the the `Clock` implementation for embedded_time
pub struct RenesasClock<const TIMER_FREQUENCY: usize> {}

impl<const TIMER_FREQUENCY: usize> Clock for RenesasClock<TIMER_FREQUENCY> {
    type T = u64;

    const SCALING_FACTOR: embedded_time::rate::Fraction = embedded_time::rate::Fraction::new(1, TIMER_FREQUENCY as u32);
    
    fn try_now(&self) -> Result<embedded_time::Instant<Self>, embedded_time::clock::Error> {
        Ok(Instant::new(DRIVER.now()))
    }
}

/// Keeps track of the underflow events for the AGT0 timer. Implements the
/// `embassy_time_driver::Driver` trait to provide timekeeping functionality.
/// TODO: Add queue for alarms in the future. See
/// https://github.com/embassy-rs/embassy/blob/main/embassy-stm32/src/time_driver.rs
/// for inspiration.
pub struct RenesasDriver {
    period: AtomicU32,
}

impl RenesasDriver {

    /// Handles hardware initialization for the timer.
    pub fn init(&'static self, timer: T) -> Self{
        #[cfg(feature = "agt0")]
        cortex_m::interrupt::free(|cs| {
            power::enable_agt0(cs);
        });

        register_interrupt(interrupt::IEL95, UNDERFLOW_EVENT);

        unsafe {
            // Set the CCRA register to 30000 counts, AGT0 counts down
            //timer.agtcma().modify(|w| w.set(65535 - 30000)); 
            //timer.agtcmsr().modify(|w| w.tcmea().set(Tcmea::_1));
            timer.agtcr().modify(|w| w.tstart().set(Tstart::_1));
        }

        cortex_m::interrupt::free(|cs| {
            TIMER.borrow(cs).replace(Some(timer));
        });

        RenesasDriver {
            period: AtomicU32::new(0),
        }
    }
}

embassy_time_driver::time_driver_impl!(
    static DRIVER: RenesasDriver = RenesasDriver {
        period: AtomicU32::new(0)
    }
);

impl Driver for RenesasDriver {
    /// Note: we are using an old commit of the embassy-time-driver crate:
    /// https://github.com/embassy-rs/embassy/tree/68c823881262989b2ef462d6d4736cc886598b50

    fn now(&self) -> u64 {
        let period = DRIVER.period.load(Ordering::Relaxed);
        unsafe {
            let count = ra4m2_pac::AGT0.agt().read().get();
            let total_ticks = period as u64 * OVERFLOW_COUNT as u64 + count as u64; 
            const US_PER_SEC: u64 = 1_000_000;
            let ticks_per_second: u64 = TIMER_CLOCK_FREQ.load(Ordering::Relaxed) as u64;
            total_ticks * US_PER_SEC / ticks_per_second
        }
    }

    unsafe fn allocate_alarm(&self) -> Option<embassy_time_driver::AlarmHandle> {
        None
    }

    fn set_alarm_callback(&self, _alarm: embassy_time_driver::AlarmHandle, _callback: fn(*mut ()), _ctx: *mut ()) {
        // TODO: add alarms later
    }

    fn set_alarm(&self, _alarm: embassy_time_driver::AlarmHandle, _timestamp: u64) -> bool {
        // TODO: add alarms later
        return false;
    }
}

pub fn init(timer: T, clock_freq: u32) {
    DRIVER.init(timer);
    TIMER_CLOCK_FREQ.store(clock_freq, Ordering::Relaxed);
}