use defmt::Format;
/// This module is for the clock generation circuit (CGC) on the RA4M2 MCU.

use ra4m2_pac::{sysc::{sckdivcr::{Ick, Pckb, Pckd, Rsv}, sckscr::Cksel}, RegisterValue};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Format)]
#[repr(u8)]
pub enum ClockSource {
    HOCO = 0, // High-speed on-chip oscillator
    MOCO = 1, // Medium-speed on-chip oscillator
    LOCO = 2, // Low-speed on-chip oscillator
    MainClockOsc = 3,  // Main clock oscillator
    SubClockOsc = 4, // Sub-clock oscillator
    PLL = 5, // Phase-locked loop
}

impl Into<Cksel> for ClockSource {
    fn into(self) -> Cksel {
        match self {
            ClockSource::HOCO => Cksel::_000,
            ClockSource::MOCO => Cksel::_001,
            ClockSource::LOCO => Cksel::_010,
            ClockSource::MainClockOsc => Cksel::_011,
            ClockSource::SubClockOsc => Cksel::_100,
            ClockSource::PLL => Cksel::_101,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ClockDividers {
    Div1 = 0, // No division
    Div2 = 1, // Divide by 2
    Div4 = 2, // Divide by 4
    Div8 = 3, // Divide by 8
    Div16 = 4, // Divide by 16
    Div32 = 5, // Divide by 32
}

impl From<u8> for ClockDividers {
    fn from(value: u8) -> Self {
        match value {
            0 => ClockDividers::Div1,
            1 => ClockDividers::Div2,
            2 => ClockDividers::Div4,
            3 => ClockDividers::Div8,
            4 => ClockDividers::Div16,
            5 => ClockDividers::Div32,
            _ => panic!("Invalid clock divider value"),
        }
    }
}

impl Into<u8> for ClockDividers {
    fn into(self) -> u8 {
        match self {
            ClockDividers::Div1 => 0,
            ClockDividers::Div2 => 1,
            ClockDividers::Div4 => 2,
            ClockDividers::Div8 => 3,
            ClockDividers::Div16 => 4,
            ClockDividers::Div32 => 5,
        }
    }
}

impl Into<Ick> for ClockDividers {
    fn into(self) -> Ick {
        match self {
            ClockDividers::Div1 => Ick::_000,
            ClockDividers::Div2 => Ick::_001,
            ClockDividers::Div4 => Ick::_010,
            ClockDividers::Div8 => Ick::_011,
            ClockDividers::Div16 => Ick::_100,
            ClockDividers::Div32 => Ick::_101,
        }
    }
}

impl Into<Pckb> for ClockDividers {
    fn into(self) -> Pckb {
        match self {
            ClockDividers::Div1 => Pckb::_000,
            ClockDividers::Div2 => Pckb::_001,
            ClockDividers::Div4 => Pckb::_010,
            ClockDividers::Div8 => Pckb::_011,
            ClockDividers::Div16 => Pckb::_100,
            ClockDividers::Div32 => Pckb::_101,
        }
    }
}

impl Into<Rsv> for ClockDividers {
    fn into(self) -> Rsv {
        match self {
            ClockDividers::Div1 => Rsv::_000,
            ClockDividers::Div2 => Rsv::_001,
            ClockDividers::Div4 => Rsv::_010,
            ClockDividers::Div8 => Rsv::_011,
            ClockDividers::Div16 => Rsv::_100,
            ClockDividers::Div32 => Rsv::_101,
        }
    }
}

impl Into<Pckd> for ClockDividers {
    fn into(self) -> Pckd {
        match self {
            ClockDividers::Div1 => Pckd::_000,
            ClockDividers::Div2 => Pckd::_001,
            ClockDividers::Div4 => Pckd::_010,
            ClockDividers::Div8 => Pckd::_011,
            ClockDividers::Div16 => Pckd::_100,
            ClockDividers::Div32 => Pckd::_101,
        }
    }
}



/// Configuration for the system clock dividers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SystemClockDividerConfig {
    pub pckb: ClockDividers,
    pub pckd: ClockDividers,
    pub pcka: ClockDividers,
    pub rsv: ClockDividers,
    pub ick: ClockDividers,
    pub _fck: ClockDividers,
}

impl Default for SystemClockDividerConfig {
    fn default() -> Self {
        SystemClockDividerConfig {
            pckb: ClockDividers::Div4,
            pckd: ClockDividers::Div4,
            pcka: ClockDividers::Div4,
            rsv: ClockDividers::Div4,
            ick: ClockDividers::Div4,
            _fck: ClockDividers::Div4,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SystemClockConfig {
    pub system_clock_divider: SystemClockDividerConfig,
    pub external_oscillator: u32,
    pub clock_source: ClockSource,
}

/// Represents the system clock for the RA4M2 MCU. The MAIN_CLOCK_FREQ is the frequency
/// of the external oscillator, i.e. 24MHz for the RA4M2 development kit.

#[repr(align(4))]
pub struct SystemClock {
    sysc: ra4m2_pac::Sysc,
    config: SystemClockConfig,
}

impl SystemClock {
    pub fn new(sysc: ra4m2_pac::Sysc, config: SystemClockConfig) -> Self {
        SystemClock { sysc, config }
    }

    pub fn get_system_clock_src(&self) -> ClockSource {
    // Read the SCKSCR register to determine the clock source
        unsafe {
            let sckscr = self.sysc.sckscr().read().cksel().get().0;
            match sckscr {
                0 => ClockSource::HOCO,
                1 => ClockSource::MOCO,
                2 => ClockSource::LOCO,
                3 => ClockSource::MainClockOsc,
                4 => ClockSource::SubClockOsc,
                5 => ClockSource::PLL,
                _ => panic!("Invalid clock source selected"),
            }
        }
    }

    pub fn get_clk_freq_divider(&self) -> SystemClockDividerConfig {
        unsafe {
            SystemClockDividerConfig {
                pckd: self.sysc.sckdivcr().read().pckd().get().0.into(),
                pckb: self.sysc.sckdivcr().read().pckb().get().0.into(),
                pcka: self.sysc.sckdivcr().read().pcka().get().0.into(),
                rsv: self.sysc.sckdivcr().read().rsv().get().0.into(), // note, set these to the same value as pkcb
                ick: self.sysc.sckdivcr().read().ick().get().0.into(),
                _fck: self.sysc.sckdivcr().read().fck().get().0.into(),
            }
        }
    }

    pub fn get_system_clk_freq(&self) -> u32 {
        // The chip resets to the MOCO clock source by default
        let freq = match self.get_system_clock_src() {
            ClockSource::HOCO => todo!(), 
            ClockSource::MOCO => 8_000_000,   
            ClockSource::LOCO => todo!(),       
            ClockSource::MainClockOsc => self.config.external_oscillator,
            ClockSource::SubClockOsc => todo!(), 
            ClockSource::PLL => todo!(),    
        };

        let clk_div = self.get_clk_freq_divider();
        let shift: u8 = clk_div.ick.into();

        freq / (1 << shift) as u32
    }    

    pub fn _enable_clock_write(&mut self) {
        // Enable write access to the clock control registers by setting PRC0 in the PRCR register
        unsafe {
            self.sysc.prcr().modify(|w| {
                // Datasheet says to write the key and value as a single value, 
                // and we need to set PRC0 to 1 to allow write access
                w.set_raw(0xA500 | 0x01)
            });
        }
    }

    pub fn _disable_clock_write(&mut self) {
        // Disable write access to the clock control registers by clearing PRC0 in the PRCR register
        unsafe {
            self.sysc.prcr().modify(|w| {
                // Datasheet says to write the key and value as a single value, 
                // and we need to set PRC0 to 0 to disable write access
                w.set_raw(0xA500 & !0x01)
            });
        }
    }

    pub fn set_system_clk_divder(&mut self, divider: ClockDividers) -> &mut Self {
        // Set the clock divider for the system clock, see PRCR register for write access
        // details.
        self.config.system_clock_divider.ick = divider; // Update the config
        cortex_m::interrupt::free(|_| {
            unsafe {
                self._enable_clock_write();
                self.sysc.sckdivcr().modify(|w| {
                    w.ick().set(divider.into())
                });
                self._disable_clock_write();
            }
        });
        self
    }

    pub fn set_system_clk_src(&mut self, src: ClockSource) -> &mut Self {
        // Set the clock source for the system clock
        self.config.clock_source = src; // Update the config
        cortex_m::interrupt::free(|_| {
            unsafe {
                self._enable_clock_write();
                self.sysc.sckscr().modify(|w| {
                    w.cksel().set(src.into())
                });
                self._disable_clock_write();
            }
        });
        self
    }

    pub fn set_pclkb_divider(&mut self, divider: ClockDividers) -> &mut Self {
        // Set the PCLKB clock divider
        self.config.system_clock_divider.pckb = divider; // Update the config
        cortex_m::interrupt::free(|_| {
            unsafe {
                self._enable_clock_write();
                self.sysc.sckdivcr().modify(|w| {
                    w.pckb().set(divider.into()).rsv().set(divider.into())
                });
                self._disable_clock_write();
            }
        });
        self
    }

    pub fn set_pclkd_divider(&mut self, divider: ClockDividers) -> &mut Self {
        // Set the PCLKA clock divider
        self.config.system_clock_divider.pckd = divider; // Update the config
        cortex_m::interrupt::free(|_| {
            unsafe {
                self._enable_clock_write();
                self.sysc.sckdivcr().modify(|w| {
                    w.pckd().set(divider.into())
                });
                self._disable_clock_write();
            }
        });
        self
    }
}
