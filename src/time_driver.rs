use embassy_time::TICK_HZ;
use embassy_time_driver::Driver;
use ra4m2_pac::{agt0::agtcr::Tstart};
use core::{cell::RefCell, sync::atomic::{compiler_fence, AtomicU32, Ordering}};
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

// 3MHz, this is needed to reconcile the timer dividers, core clock, and make sure the embassy TICK_HZ is all
// aligned. I can't make AGT0 run at exactly 1MHz. 
static TIMER_CLOCK_FREQ: AtomicU32 = AtomicU32::new(3_000_000); 

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
#[repr(align(4))]
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

embassy_time_driver::time_driver_impl!(
    static DRIVER: RenesasDriver = RenesasDriver {
        period: AtomicU32::new(0)
    }
);

impl Driver for RenesasDriver {
    /// Note: we are using an old commit of the embassy-time-driver crate:
    /// https://github.com/embassy-rs/embassy/tree/68c823881262989b2ef462d6d4736cc886598b50

    fn now(&self) -> u64 {
        cortex_m::interrupt::free(|_cs| {
            let period = DRIVER.period.load(Ordering::Relaxed);
            let timer_ticks_per_second = TIMER_CLOCK_FREQ.load(Ordering::Relaxed);
            compiler_fence(Ordering::Acquire);
            let div = timer_ticks_per_second / TICK_HZ as u32;
            // Compute the number of timer ticks in an embassy time tick rate.
            //let count = ra4m2_pac::AGT0.agt().read().get() / div as u16;
            // TODO: Revisit this. I think the issue is that now() was being called from different spots and the 
            // Atomic had not been update, but the tick count was. This might have been leading to non-sequential 
            // u64 times that were causing u64 errors when subtracted. It is close-ish, but I need to study 
            // the STM32, RP2040, and nRF a bit more to better understand the time-keeping.
            let total_ticks = period as u64 * (OVERFLOW_COUNT / div as u16) as u64; 
            total_ticks
        })
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