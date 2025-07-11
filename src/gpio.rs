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

pub trait AnyPin {}

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

    use crate::{gpio::{AlternateFunction, AnalogInput, DrainControl, DriveMode, HighZ, Input, InterruptEnable, InterruptEvent, Output, PinState, PortDirection, PortMode, PullDown, PullUp, PullUpMode, PushPull}};

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

    #[repr(align(4))]
    pub struct Pin<S: PinState, const N: u8> {
        _p: PhantomData<S>,
    }

    impl<S: PinState + Sized, const N: u8> Pin<S, N> {
        pub fn into_output_push_pull(self, drive_mode: DriveMode) -> Pin<Output<PushPull>, N> {
            crate::pfsel::port4::set_pin_function(
                N,
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

            Pin {
                _p: PhantomData,
            }
        }

        pub fn into_alternate_function(self, function: PinFunction) -> Pin<Output<AlternateFunction>, N> {
            crate::pfsel::port4::set_pin_function(
                N,
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

                Pin {
                    _p: PhantomData,
                }
        }

        pub fn into_input_pull_up(&self) -> Pin<Input<PullUp>, N> {
            crate::pfsel::port4::set_pin_function(
                N,
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

            Pin {
                _p: PhantomData,
            }
        }

        pub fn into_input_pull_down(&self) -> Pin<Input<PullDown>, N> {
            crate::pfsel::port4::set_pin_function(
                N,
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

            Pin {
                _p: PhantomData,
            }
        }
    }

    impl<const N: u8> embedded_hal::digital::v2::OutputPin for Pin<Output<PushPull>, N> {
        type Error = core::convert::Infallible;

        fn set_high(&mut self) -> Result<(), Self::Error> {
            crate::pfsel::port4::set_pin_value(N, crate::gpio::OutputValue::High);
            Ok(())
        }

        fn set_low(&mut self) -> Result<(), Self::Error> {
            crate::pfsel::port4::set_pin_value(N, crate::gpio::OutputValue::Low);
            Ok(())
        }
    }

    impl<const N: u8> embedded_hal::digital::v2::InputPin for Pin<Input<PullUp>, N> {
        type Error = core::convert::Infallible;

        fn is_high(&self) -> Result<bool, Self::Error> {
            Ok(crate::pfsel::port4::get_pin_value(N))
        }

        fn is_low(&self) -> Result<bool, Self::Error> {
            Ok(!crate::pfsel::port4::get_pin_value(N))
        }
    }

    impl<const N: u8> embedded_hal::digital::v2::InputPin for Pin<Input<PullDown>, N> {
        type Error = core::convert::Infallible;

        fn is_high(&self) -> Result<bool, Self::Error> {
            Ok(crate::pfsel::port4::get_pin_value(N))
        }

        fn is_low(&self) -> Result<bool, Self::Error> {
            Ok(!crate::pfsel::port4::get_pin_value(N))
        }
    }
    
    pub struct Port4Pins {
        pub p00: Pin<Output<HighZ>, 0>,
        pub p01: Pin<Output<HighZ>, 1>,
        pub p02: Pin<Output<HighZ>, 2>,
        pub p03: Pin<Output<HighZ>, 3>,
        pub p04: Pin<Output<HighZ>, 4>,
        pub p05: Pin<Output<HighZ>, 5>,
        pub p06: Pin<Output<HighZ>, 6>,
        pub p07: Pin<Output<HighZ>, 7>,
        pub p08: Pin<Output<HighZ>, 8>,
        pub p09: Pin<Output<HighZ>, 9>,
        pub p10: Pin<Output<HighZ>, 10>,
        pub p11: Pin<Output<HighZ>, 11>,
        pub p12: Pin<Output<HighZ>, 12>,
        pub p13: Pin<Output<HighZ>, 13>,
        pub p14: Pin<Output<HighZ>, 14>,
        pub p15: Pin<Output<HighZ>, 15>,
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
                p00: Pin { _p: PhantomData },
                p01: Pin { _p: PhantomData },
                p02: Pin { _p: PhantomData },
                p03: Pin { _p: PhantomData },
                p04: Pin { _p: PhantomData },
                p05: Pin { _p: PhantomData },
                p06: Pin { _p: PhantomData },
                p07: Pin { _p: PhantomData },
                p08: Pin { _p: PhantomData },
                p09: Pin { _p: PhantomData },
                p10: Pin { _p: PhantomData },
                p11: Pin { _p: PhantomData },
                p12: Pin { _p: PhantomData },
                p13: Pin { _p: PhantomData },
                p14: Pin { _p: PhantomData },
                p15: Pin { _p: PhantomData },
            }
        }
    }
}










