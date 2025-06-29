use ra4m2_pac::Port1;

use crate::gpio4::PinFunctionPort4;
use crate::pfsel::set_pin_function_port4;

pub struct Demo {
    _port1: Port1,
}

// This is a demo module that configures the pins for I2C communication
// TODO: Remove this module later. 
impl Demo {
    pub fn new(port1: Port1) -> Self {
        Demo { _port1: port1 }
    }

    pub fn configure_pins(&mut self) {
        // Configure GPIO for I2C, P112 = SDA, P113 = SCL

        set_pin_function_port4(0, PinFunctionPort4::IIC, true, false, true);
        set_pin_function_port4(1, PinFunctionPort4::IIC, true, false, true);


        // Additional configuration can be added here
    }
}