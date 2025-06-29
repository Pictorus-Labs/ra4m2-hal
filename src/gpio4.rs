use core::cell::RefCell;

use ra4m2_pac::{pfs::P1Pfs, ClusterRegisterArray, Port1, Port0, Reg};

static PORT4: cortex_m::interrupt::Mutex<RefCell<Option<Port1>>> = cortex_m::interrupt::Mutex::new(RefCell::new(None));

pub struct Port4 {
    port4: ra4m2_pac::Port1,
}

impl Port4 {
    pub fn new(port4: ra4m2_pac::Port1) -> Self {
        cortex_m::interrupt::free(|cs| {
            PORT4.borrow(cs).replace(Some(port4));
        });
        Port4 { port4 }
    }
}

pub trait Port {}

pub enum Ports {
    Port0 = 0,
    Port1 = 1,
    Port2 = 2,
    Port3 = 3,
    Port4 = 4,
    Port5 = 5,
    Port6 = 6,
    Port7 = 7,
    Port8 = 8,
}

impl Into<u8> for Ports {
    fn into(self) -> u8 {
        match self {
            Ports::Port0 => 0,
            Ports::Port1 => 1,
            Ports::Port2 => 2,
            Ports::Port3 => 3,
            Ports::Port4 => 4,
            Ports::Port5 => 5,
            Ports::Port6 => 6,
            Ports::Port7 => 7,
            Ports::Port8 => 8,
        }
    }
}

pub enum PinFunctionPort4 {
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

impl Into<u8> for PinFunctionPort4 {
    fn into(self) -> u8 {
        match self {
            PinFunctionPort4::GPIO => 0,
            PinFunctionPort4::AGT => 1,
            PinFunctionPort4::GPTA => 2,
            PinFunctionPort4::GPTB => 3,
            PinFunctionPort4::SCIA => 4,
            PinFunctionPort4::SCIB => 5,
            PinFunctionPort4::IIC => 7,
            PinFunctionPort4::RTC => 9,
            PinFunctionPort4::ADC => 10,
            PinFunctionPort4::CTSU => 12,
            PinFunctionPort4::CAN => 16,
            PinFunctionPort4::SSIE => 18,
            PinFunctionPort4::USBFS => 19,
            PinFunctionPort4::SDHI => 21,
        }
    }
}

struct Pin {
    port: Ports,
    pin: u8,
}

pub enum DriveMode {
    Low = 0,
    Middle = 1,
    High = 4,
}

pub enum OutputMode {
    PushPull = 0,
    OpenDrain = 1,
}

pub enum PullUpMode {
    Disabled = 0,
    Enabled = 1,
}

pub enum PortDirection {
    Input = 0,
    Output = 1,
}

pub struct P100<MODE> {
    _mode: core::marker::PhantomData<MODE>,
}

// struct Gpio1 {
//     port: Port1,
//     pfs: &'static ClusterRegisterArray<Reg<P1Pfs_SPEC, RW>, 6, 4>,

// }

// impl Gpio1 {
//     pub fn new(port: Port1, pfs: &'static ClusterRegisterArray<P1Pfs, 6, 4>) -> Self {
//         Gpio1 { port, pfs }
//     }

    
// }
