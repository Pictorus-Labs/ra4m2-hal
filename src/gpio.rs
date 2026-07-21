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
    High = 3,
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

/// Peripheral function encoding for the PSEL field of the PmnPFS registers.
/// The encoding is the same for every port, but which functions are actually
/// available on a given pin varies — check the "Peripheral Select Settings"
/// tables in the RA4M2 User's Manual for the pin in question.
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

/// Generates the GPIO module for one port. Invoked for every port via
/// `for_each_port!` in `port_map.rs` — the single source of truth for which
/// pins exist. Entries are `(field, pin_number, <pfs accessor tokens>)`; the
/// accessor tokens are consumed by the `pfs_port!` callback in `pfsel.rs` and
/// ignored here.
///
/// Note the PAC reuses one register-block type per group of ports with an
/// identical layout: `ra4m2_pac::Port0` is the type of PORT0/5/6/7 and
/// `ra4m2_pac::Port1` the type of PORT1/2/3/4 (same registers, different base
/// address). The constructor therefore cannot verify that the peripheral you
/// pass in is the matching `PORTn` instance — pass the right one.
macro_rules! gpio_port {
    (
        $feature:literal, $mod_name:ident, $port_struct:ident, $pins_struct:ident, $pac_ty:ty,
        [ $( ($field:ident, $n:literal, $($_acc:tt)+) ),+ $(,)? ]
    ) => {
        #[cfg(feature = $feature)]
        pub mod $mod_name {
            use core::marker::PhantomData;

            use crate::gpio::{
                AlternateFunction, AnalogInput, DrainControl, DriveMode, Floating, HighZ, Input,
                InputState, InterruptEnable, InterruptEvent, OpenDrain, Output, PinState,
                PortDirection, PortMode, PullDown, PullUp, PullUpMode, PushPull,
            };
            pub use crate::gpio::PinFunction;

            #[derive(Debug, Clone, Copy)]
            pub struct Pin<S: PinState, const N: u8> {
                _p: PhantomData<S>,
            }

            impl<S: PinState + Sized, const N: u8> Pin<S, N> {
                pub fn into_output_push_pull(self, drive_mode: DriveMode) -> Pin<Output<PushPull>, N> {
                    crate::pfsel::$mod_name::set_pin_function(
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

                    Pin { _p: PhantomData }
                }

                pub fn into_output_open_drain(self, drive_mode: DriveMode) -> Pin<Output<OpenDrain>, N> {
                    crate::pfsel::$mod_name::set_pin_function(
                        N,
                        PortDirection::Output,
                        PullUpMode::Disabled,
                        DrainControl::OpenDrain,
                        drive_mode,
                        InterruptEvent::DontCare,
                        InterruptEnable::Disabled,
                        AnalogInput::Disabled,
                        PinFunction::GPIO,
                        PortMode::Normal,
                    );

                    Pin { _p: PhantomData }
                }

                pub fn into_alternate_function(self, function: PinFunction) -> Pin<Output<AlternateFunction>, N> {
                    crate::pfsel::$mod_name::set_pin_function(
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

                    Pin { _p: PhantomData }
                }

                pub fn into_input_pull_up(self) -> Pin<Input<PullUp>, N> {
                    crate::pfsel::$mod_name::set_pin_function(
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

                    Pin { _p: PhantomData }
                }

                pub fn into_input_pull_down(self) -> Pin<Input<PullDown>, N> {
                    crate::pfsel::$mod_name::set_pin_function(
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

                    Pin { _p: PhantomData }
                }

                pub fn into_input_floating(self) -> Pin<Input<Floating>, N> {
                    crate::pfsel::$mod_name::set_pin_function(
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

                    Pin { _p: PhantomData }
                }
            }

            impl<S: PinState, const N: u8> embedded_hal::digital::ErrorType for Pin<S, N> {
                type Error = core::convert::Infallible;
            }

            impl<const N: u8> embedded_hal::digital::OutputPin for Pin<Output<PushPull>, N> {
                fn set_high(&mut self) -> Result<(), Self::Error> {
                    crate::pfsel::$mod_name::set_pin_value(N, crate::gpio::OutputValue::High);
                    Ok(())
                }

                fn set_low(&mut self) -> Result<(), Self::Error> {
                    crate::pfsel::$mod_name::set_pin_value(N, crate::gpio::OutputValue::Low);
                    Ok(())
                }
            }

            impl<const N: u8> embedded_hal::digital::OutputPin for Pin<Output<OpenDrain>, N> {
                fn set_high(&mut self) -> Result<(), Self::Error> {
                    crate::pfsel::$mod_name::set_pin_value(N, crate::gpio::OutputValue::High);
                    Ok(())
                }

                fn set_low(&mut self) -> Result<(), Self::Error> {
                    crate::pfsel::$mod_name::set_pin_value(N, crate::gpio::OutputValue::Low);
                    Ok(())
                }
            }

            impl<S: InputState, const N: u8> embedded_hal::digital::InputPin for Pin<Input<S>, N> {
                fn is_high(&mut self) -> Result<bool, Self::Error> {
                    Ok(crate::pfsel::$mod_name::get_pin_value(N))
                }

                fn is_low(&mut self) -> Result<bool, Self::Error> {
                    Ok(!crate::pfsel::$mod_name::get_pin_value(N))
                }
            }

            pub struct $pins_struct {
                $( pub $field: Pin<Output<HighZ>, $n>, )+
            }

            pub struct $port_struct {
                _port: $pac_ty,
            }

            impl $port_struct {
                pub fn new(port: $pac_ty) -> Self {
                    $port_struct { _port: port }
                }

                /// Consumes the port and hands out one singleton per pin. Taking
                /// `self` means this can only be called once, so typed pins can't
                /// be duplicated.
                pub fn split(self) -> $pins_struct {
                    $pins_struct {
                        $( $field: Pin { _p: PhantomData }, )+
                    }
                }
            }
        }
    };
}

// The per-port pin lists live in `port_map.rs`, shared with `pfsel.rs` so the
// pin set and its register mapping can't drift apart.
use crate::port_map::for_each_port;

for_each_port!(gpio_port);
