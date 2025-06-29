use core::{cell::RefCell, panic};

use cortex_m::interrupt::InterruptNumber;
use ra4m2_pac::NoBitfieldReg;

static ICU: cortex_m::interrupt::Mutex<RefCell<Option<ra4m2_pac::Icu>>> = cortex_m::interrupt::Mutex::new(RefCell::new(None));

static CLEAR_INTERRUPT: u32 = 0xFFFE_FFFF; // Mask to clear the interrupt

static INTERRUPT_EVENTS: u16 = 96; // Example event number for interrupts

/// Interrupt Control Unit (ICU) structure
pub struct Icu {}

impl Icu {
    pub fn init(icu: ra4m2_pac::Icu) {
        cortex_m::interrupt::free(|cs| {
            ICU.borrow(cs).replace(Some(icu));
        });
    }
}

/// Register an interrupt handler for a specific event
pub fn register_interrupt<T: InterruptNumber>(interrupt: T, event: u16) {
    // Register an interrupt handler for a specific event
    if interrupt.number() < INTERRUPT_EVENTS {
        cortex_m::interrupt::free(|cs| {
            if let Some(icu) = ICU.borrow(cs).borrow_mut().as_mut() {
                unsafe {
                    cortex_m::peripheral::NVIC::unmask(interrupt);
                    icu.ielsr().get(interrupt.number() as usize).modify(|w| w.set(event as u32));
                }
            }
        });
    } else {
        panic!("Event number out of range");
    }
}

/// Clear the interrupt for a specific event
pub fn clear_interrupt<T: InterruptNumber>(interrupt: T) {
    // Clear the interrupt for a specific event
    if interrupt.number() < INTERRUPT_EVENTS {
        cortex_m::interrupt::free(|cs| {
            if let Some(icu) = ICU.borrow(cs).borrow_mut().as_mut() {
                unsafe {
                    let mut contents = icu.ielsr().get(interrupt.number() as usize).read().get();
                    contents &= CLEAR_INTERRUPT;
                    icu.ielsr().get(interrupt.number() as usize).modify(|w| w.set(contents));
                }
            }
        });
    } else {
        panic!("Event number out of range");
    }
}

