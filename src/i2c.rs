use ra4m2_pac::{iic0::{iccr1::{Ice, Iicrst}, iccr2::{Sp, St}, icmr1::{Bc, Bcwp}, icser::Sar0E, icsr2::{Nackf, Stop}}, Iic0, RegisterValue};

use crate::power;

/// I2C (Inter-Integrated Circuit) driver for RA4M2 microcontroller
pub struct I2c {
    iic0: Iic0,
}

pub enum I2cError {
    BusBusy,
    SlaveNotResponding,
}

impl I2c {
    /// Creates a new I2C instance with the given IIC0 peripheral.
    pub fn new(iic0: Iic0) -> Self {
        cortex_m::interrupt::free(|cs| {
            power::enable_i2c0(cs); 
        });
        
        I2c { iic0 }
    }

    /// Initializes the I2C settings for the given slave address.
    pub fn _initialize_settings(&mut self, address: u8) {
        // https://www.renesas.com/en/document/man/ra4m2-group-users-manual-hardware?r=1469026
        // See page 1004 of the RA4M2 manual for initialization flowchart
        unsafe {
            self.iic0.iccr1().modify(|w| w.ice().set(Ice::_0));
            self.iic0.iccr1().modify(|w| w.iicrst().set(Iicrst::_1));
            self.iic0.iccr1().modify(|w| w.ice().set(Ice::_1));
            
            // TODO: We are just writing to the IIC register for slave address 0, 7 bit only
            self.iic0.sarl().get(0).modify(|w| w.sva().set(address));
            self.iic0.saru().get(0).modify(|w| w.sva().set(0));
            self.iic0.icser().modify(|w| w.sar0e().set(Sar0E::_1));

            // Not sure icmr1, 2, 3 need to be set for basic operations, bit counter defaults to 0?
            self.iic0.icmr1().modify(|w| w.bcwp().set(Bcwp::_1).bc().set(Bc::_000));
            //self.iic0.icmr2().modify(|w| todo!()); 
            //self.iic0.icmr3().modify(|w| todo!());

            // TODO: Interrupts, just going to poll for now
            //self.iic0.icier().modify(|w| todo!());

            self.iic0.iccr1().modify(|w| w.iicrst().set(Iicrst::_0));
        }
    }

    /// Writes data to the I2C slave device at the specified address.
    pub fn write(&mut self, address: u8, data: &[u8]) -> Result<(), I2cError> {
        self._initialize_settings(address);

        unsafe {
            let bus_busy = self.iic0.iccr2().read().bbsy().get().0;
            let mut timeout = 0;
            while bus_busy != 0 {
                if timeout > 10000 {
                    return Err(I2cError::BusBusy);
                }
                timeout += 1;
            }

            self.iic0.iccr2().modify(|w| w.st().set(St::_1));

            let mut nackf_issue = false;

            for byte in data {
                if self.iic0.icsr2().read().nackf().get().0 == 0 {
                    self.iic0.icdrt().modify(|w| w.set_raw(address << 1 | 0x00)); // Write operation, LSB is 0
                    if self.iic0.icsr2().read().tdre().get().0 == 1 {
                        // transmit data
                        self.iic0.icdrt().modify(|w| w.set_raw(*byte));
                    }
                }else{
                    nackf_issue = true;
                    break;
                }
            }

            if nackf_issue == false {
                while self.iic0.icsr2().read().tend().get().0 != 1{
                    // Wait for transmission to complete
                    if timeout > 10000 {
                        return Err(I2cError::SlaveNotResponding);
                    }
                    timeout += 1;
                }
            }

            self.iic0.icsr2().modify(|w| w.stop().set(Stop::_0));
            self.iic0.iccr2().modify(|w| w.sp().set(Sp::_1));
            
            while self.iic0.icsr2().read().stop().get().0 == 0 {
                // Wait for stop condition to be set
            }

            self.iic0.icsr2().modify(|w| w.nackf().set(Nackf::_0)); // Clear NACK flag
            self.iic0.icsr2().modify(|w| w.stop().set(Stop::_0));

            if nackf_issue {
                return Err(I2cError::SlaveNotResponding);
            }

            Ok(())
        }
    }
}