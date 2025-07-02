use crate::sealed;

pub enum Ports {
    Port0 = 0,
    Port1 = 1,
    Port2 = 2,
    Port3 = 3,
    Port4 = 4,
    Port5 = 5,
    Port6 = 6,
    Port7 = 7,
}

pub enum PortDirection {
    Input = 0,
    Output = 1,
}

pub enum OutputValue {
    Low = 0,
    High = 1,
}

pub enum PullUpMode {
    Disabled = 0,
    Enabled = 1,
}

pub enum DrainControl {
    PushPull = 0,
    OpenDrain = 1,
}

pub enum DriveMode {
    Low = 0,
    Middle = 1,
    High = 4,
}

pub enum InterruptEvent {
    DontCare = 0,
    RisingEdge = 1,
    FallingEdge = 2,
    BothEdges = 3,
}

pub enum InterruptEnable {
    Disabled = 0,
    Enabled = 1,
}

pub enum AnalogInput {
    Disabled = 0,
    Enabled = 1,
}

pub enum PortMode {
    Normal = 0,
    Alternate = 1,
}

pub trait PinState: sealed::Sealed {}
pub trait InputState: sealed::Sealed {}
pub trait OutputState: sealed::Sealed {}

pub struct Output<S: OutputState> {
    _state: core::marker::PhantomData<S>,
}

pub struct PushPull {}
pub struct OpenDrain {}
pub struct AlternateFunction {}
pub struct HighZ {}

impl<S: OutputState> PinState for Output<S> {}
impl<S: OutputState> sealed::Sealed for Output<S> {}

impl OutputState for PushPull {}
impl OutputState for OpenDrain {}
impl OutputState for AlternateFunction {}
impl OutputState for HighZ {}
impl sealed::Sealed for PushPull {}
impl sealed::Sealed for OpenDrain {}
impl sealed::Sealed for AlternateFunction {}
impl sealed::Sealed for HighZ {}

pub struct Input<S: InputState> {
    _state: core::marker::PhantomData<S>,
}
pub struct PullUp {}
pub struct PullDown {}
pub struct Floating {}

impl<S: InputState> PinState for Input<S> {}
impl<S: InputState> sealed::Sealed for Input<S> {}

impl InputState for PullUp {}
impl InputState for PullDown {}
impl InputState for Floating {}
impl sealed::Sealed for PullUp {}
impl sealed::Sealed for PullDown {}
impl sealed::Sealed for Floating {}

/// Module for Port 4
#[cfg(feature = "port4")]
pub mod port4 {
    use core::{cell::RefCell, marker::PhantomData};
    use ra4m2_pac::Port1;

    use crate::{gpio::{AlternateFunction, AnalogInput, DrainControl, DriveMode, HighZ, Input, InterruptEnable, InterruptEvent, Output, PinState, PortDirection, PortMode, PullDown, PullUp, PullUpMode, PushPull}, i2c::{I2cSCLPin, I2cSDAPin}};

    // Note Port4 is a struct of r4m2_pac::Port1
    static PORT4: cortex_m::interrupt::Mutex<RefCell<Option<Port1>>> = cortex_m::interrupt::Mutex::new(RefCell::new(None));

    #[derive(Debug, Clone, Copy)]
    pub enum PinFunction {
        GPIO = 0,
        AGT = 1,
        GPTA = 2,
        GPTB = 3,
        SCIA = 4,
        SCIB = 5,
        IIC = 7, 
        RTC = 9,
        ADC = 10,
        CTSU = 12,
        CAN = 16,
        SSIE = 18,
        USBFS = 19,
        SDHI = 21,
    }

    /// Macro to allocate a pin with a given name and number and a list of additional traits
    /// to help constrain the pin type.
    macro_rules! allocate_pin{
        ($pin_name:ident, $pin_number:literal $(, $trait:path)* $(,)?) => {
            pub struct $pin_name<S: PinState> {
                _p: PhantomData<S>,
            }

            impl<S: PinState> $pin_name<S> {
                pub fn into_output_push_pull(self, drive_mode: DriveMode) -> $pin_name<Output<PushPull>> {
                    crate::pfsel::port4::set_pin_function(
                        $pin_number,
                        PortDirection::Output,
                        PullUpMode::Disabled,
                        DrainControl::PushPull,
                        drive_mode,
                        InterruptEvent::DontCare,
                        InterruptEnable::Disabled,
                        AnalogInput::Disabled,
                        PinFunction::GPIO,
                        PortMode::Normal,
                    );

                    $pin_name {
                        _p: PhantomData,
                    }
                }

                pub fn into_alternate_function(self, function: PinFunction) -> $pin_name<Output<AlternateFunction>> {
                    crate::pfsel::port4::set_pin_function(
                        $pin_number,
                        PortDirection::Output,
                        PullUpMode::Disabled,
                        DrainControl::OpenDrain,
                        DriveMode::Low,
                        InterruptEvent::DontCare,
                        InterruptEnable::Disabled,
                        AnalogInput::Disabled,
                        function,
                        PortMode::Alternate,
                        );

                        $pin_name {
                            _p: PhantomData,
                        }
                }

                pub fn into_input_pull_up(&self) -> $pin_name<Input<PullUp>> {
                    crate::pfsel::port4::set_pin_function(
                        $pin_number,
                        PortDirection::Input,
                        PullUpMode::Enabled,
                        DrainControl::PushPull,
                        DriveMode::Low,
                        InterruptEvent::DontCare,
                        InterruptEnable::Disabled,
                        AnalogInput::Disabled,
                        PinFunction::GPIO,
                        PortMode::Normal,
                    );

                    $pin_name {
                        _p: PhantomData,
                    }
                }

                pub fn into_input_pull_down(&self) -> $pin_name<Input<PullDown>> {
                    crate::pfsel::port4::set_pin_function(
                        $pin_number,
                        PortDirection::Input,
                        PullUpMode::Disabled,
                        DrainControl::PushPull,
                        DriveMode::Low,
                        InterruptEvent::DontCare,
                        InterruptEnable::Disabled,
                        AnalogInput::Disabled,
                        PinFunction::GPIO,
                        PortMode::Normal,
                    );

                    $pin_name {
                        _p: PhantomData,
                    }
                }
            }

            impl embedded_hal::digital::v2::OutputPin for $pin_name<Output<PushPull>> {
                type Error = core::convert::Infallible;

                fn set_high(&mut self) -> Result<(), Self::Error> {
                    crate::pfsel::port4::set_pin_value($pin_number, crate::gpio::OutputValue::High);
                    Ok(())
                }

                fn set_low(&mut self) -> Result<(), Self::Error> {
                    crate::pfsel::port4::set_pin_value($pin_number, crate::gpio::OutputValue::Low);
                    Ok(())
                }
            }

            impl embedded_hal::digital::v2::InputPin for $pin_name<Input<PullUp>> {
                type Error = core::convert::Infallible;

                fn is_high(&self) -> Result<bool, Self::Error> {
                    Ok(crate::pfsel::port4::get_pin_value($pin_number))
                }

                fn is_low(&self) -> Result<bool, Self::Error> {
                    Ok(!crate::pfsel::port4::get_pin_value($pin_number))
                }
            }
        }
    }

    allocate_pin!(P00, 0);
    allocate_pin!(P01, 1);
    allocate_pin!(P02, 2);
    allocate_pin!(P03, 3);
    allocate_pin!(P04, 4);
    allocate_pin!(P05, 5);
    allocate_pin!(P06, 6);
    allocate_pin!(P07, 7);
    allocate_pin!(P08, 8);
    allocate_pin!(P09, 9);
    allocate_pin!(P10, 10);
    allocate_pin!(P11, 11);
    allocate_pin!(P12, 12);
    allocate_pin!(P13, 13);
    allocate_pin!(P14, 14);
    allocate_pin!(P15, 15);
    
    pub struct Port4Pins {
        pub p00: P00<Output<HighZ>>,
        pub p01: P01<Output<HighZ>>,
        pub p02: P02<Output<HighZ>>,
        pub p03: P03<Output<HighZ>>,
        pub p04: P04<Output<HighZ>>,
        pub p05: P05<Output<HighZ>>,
        pub p06: P06<Output<HighZ>>,
        pub p07: P07<Output<HighZ>>,
        pub p08: P08<Output<HighZ>>,
        pub p09: P09<Output<HighZ>>,
        pub p10: P10<Output<HighZ>>,
        pub p11: P11<Output<HighZ>>,
        pub p12: P12<Output<HighZ>>,
        pub p13: P13<Output<HighZ>>,
        pub p14: P14<Output<HighZ>>,
        pub p15: P15<Output<HighZ>>,
    }

    pub struct Port4 {
        // Note Port4 is a struct of r4m2_pac::Port1
        _port4: ra4m2_pac::Port1, 
    }

    impl Port4 {
        // Note Port4 is a struct of r4m2_pac::Port1
        pub fn new(port4: ra4m2_pac::Port1) -> Self {
            cortex_m::interrupt::free(|cs| {
                PORT4.borrow(cs).replace(Some(port4));
            });
            Port4 { _port4: port4 }
        }

        pub fn split(&self) -> Port4Pins {
            Port4Pins {
                p00: P00 { _p: PhantomData },
                p01: P01 { _p: PhantomData },
                p02: P02 { _p: PhantomData },
                p03: P03 { _p: PhantomData },
                p04: P04 { _p: PhantomData },
                p05: P05 { _p: PhantomData },
                p06: P06 { _p: PhantomData },
                p07: P07 { _p: PhantomData },
                p08: P08 { _p: PhantomData },
                p09: P09 { _p: PhantomData },
                p10: P10 { _p: PhantomData },
                p11: P11 { _p: PhantomData },
                p12: P12 { _p: PhantomData },
                p13: P13 { _p: PhantomData },
                p14: P14 { _p: PhantomData },
                p15: P15 { _p: PhantomData },
            }
        }
    }
}










