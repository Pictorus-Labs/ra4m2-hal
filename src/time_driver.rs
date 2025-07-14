use embedded_time::{rate::Fraction, Instant};
use ra4m2_pac::{agt0::agtcr::Tstart};
use core::{cell::RefCell, sync::atomic::{compiler_fence, AtomicU32, Ordering}};
use ra4m2_pac::{interrupt};
use crate::{icu::{clear_interrupt, register_interrupt}, power};

static TICK_HZ: u64 = 1_000_000; // 1 MHz, this is the embassy time tick rate

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

// 3MHz, this is needed to reconcile the timer dividers, core clock, and make sure the 1MHz TICK_HZ is all
// aligned. I can't make AGT0 run at exactly 1MHz. 
static TIMER_CLOCK_FREQ: AtomicU32 = AtomicU32::new(3_000_000); 

// The driver instance to keep track of overflow events
static DRIVER: RenesasDriver = RenesasDriver { period: AtomicU32::new(0) };

#[cfg(feature = "agt0")]
#[interrupt]
fn IEL95() {
    clear_interrupt(interrupt::IEL95);
    let current = DRIVER.period.load(Ordering::Relaxed) + 1;
    DRIVER.period.store(current, Ordering::Relaxed);
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

#[derive(Default)]
pub struct RenesasClock;

impl embedded_time::Clock for RenesasClock {
    type T = u64;

    const SCALING_FACTOR: Fraction = Fraction::new(1, TICK_HZ as u32);

    fn try_now(&self) -> Result<Instant<Self>, embedded_time::clock::Error> {
        let total_ticks = cortex_m::interrupt::free(|_cs| {
            let period = DRIVER.period.load(Ordering::Relaxed);
            let timer_ticks_per_second = TIMER_CLOCK_FREQ.load(Ordering::Relaxed);
            compiler_fence(Ordering::Acquire);
            // Compute the number of timer ticks in an embassy time tick rate.
            // TODO: figure out correct way to calculate this, still getting a u64 under / overflow
            // when the count is included.
            // let count = unsafe {
            //     // AGT are count down timers, so we need to subtract the current count from the overflow count
            //     OVERFLOW_COUNT - ra4m2_pac::AGT0.agt().read().get() as u16
            // };
            let total_ticks = period as u64 * OVERFLOW_COUNT as u64; 
            let total_ticks = total_ticks * TICK_HZ / timer_ticks_per_second as u64;
            total_ticks
        });
        Ok(Instant::new(total_ticks))
    }
}

pub fn init(timer: T, clock_freq: u32) {
    DRIVER.init(timer);
    TIMER_CLOCK_FREQ.store(clock_freq, Ordering::Relaxed);
}