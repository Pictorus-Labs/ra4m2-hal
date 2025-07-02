use embedded_hal::blocking::i2c::SevenBitAddress;
use ra4m2_pac::{iic0::{iccr1::{Ice, Iicrst}, iccr2::{Sp, St}, icmr1::{Bc, Bcwp}, icmr3::{Ackbt, Ackwp, Wait}, icser::Sar0E, icsr2::{Nackf, Stop}}, Iic0, RegisterValue};
use crate::power;

enum Direction {
    Write = 0x00,
    Read = 0x01,
}

impl Into<u8> for Direction {
    fn into(self) -> u8 {
        match self {
            Direction::Read => 0x00,
            Direction::Write => 0x01,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum I2cError {
    BusBusy,
    SlaveNotResponding,
    PeripheralNotStopped,
    TransmitBufferNotReady,
    DataNotReceived,
}

fn get_slave_address(address: u8, direction: Direction) -> u8 {
    // The slave address is shifted left by 1 bit to accommodate the read/write bit
    (address << 1) | direction as u8
}

/// Trait for I2C SDA pin
pub trait I2cSDAPin {}
/// Trait for I2C SCL pin
pub trait I2cSCLPin {}

macro_rules! define_i2c {
    ($name:ident, $IIC:ident, $power_func:ident) => {
        /// I2C (Inter-Integrated Circuit) driver for RA4M2 microcontroller
        pub struct $name<SDA: I2cSDAPin, SCL: I2cSCLPin> {
            iic: $IIC,
            _sda: SDA,
            _scl: SCL,
        }

        impl<SDA: I2cSDAPin, SCL: I2cSCLPin> $name<SDA, SCL> {
            /// Creates a new I2C instance with the given IIC0 peripheral.
            pub fn new(iic: $IIC, sda: SDA, scl: SCL) -> Self {
                cortex_m::interrupt::free(|cs| {
                    power::$power_func(cs); 
                });
                
                $name { iic, _sda: sda, _scl: scl }
            }

            /// Initializes the I2C settings for the given slave address.
            fn initialize_settings(&mut self, address: u8) {
                // https://www.renesas.com/en/document/man/ra4m2-group-users-manual-hardware?r=1469026
                // See page 1004 of the RA4M2 manual for initialization flowchart
                unsafe {
                    self.iic.iccr1().modify(|w| w.ice().set(Ice::_0));
                    self.iic.iccr1().modify(|w| w.iicrst().set(Iicrst::_1));
                    self.iic.iccr1().modify(|w| w.ice().set(Ice::_1));
                    
                    // TODO: We are just writing to the IIC register for slave address 0, 7 bit only
                    self.iic.sarl().get(0).modify(|w| w.sva().set(address));
                    self.iic.saru().get(0).modify(|w| w.sva().set(0));
                    self.iic.icser().modify(|w| w.sar0e().set(Sar0E::_1));

                    // Not sure icmr1, 2, 3 need to be set for basic operations, bit counter defaults to 0?
                    self.iic.icmr1().modify(|w| w.bcwp().set(Bcwp::_1).bc().set(Bc::_000));

                    self.iic.iccr1().modify(|w| w.iicrst().set(Iicrst::_0));
                }
            }

            fn is_bus_free(&self) -> bool {
                unsafe {
                    self.iic.iccr2().read().bbsy().get().0 == 0
                }
            }

            fn wait_for_bus(&self) -> Result<(), I2cError> {
                let mut timeout = 0;
                while !self.is_bus_free() {
                    if timeout > 10000 {
                        return Err(I2cError::BusBusy);
                    }
                    timeout += 1;
                }
                Ok(())
            }

            fn start(&mut self) {
                unsafe {
                    self.iic.iccr2().modify(|w| w.st().set(St::_1));
                }
            }

            fn stop(&mut self) {
                unsafe {
                    self.iic.iccr2().modify(|w| w.sp().set(Sp::_1));
                }
            }

            fn unstop(&mut self) {
                unsafe {
                    self.iic.iccr2().modify(|w| w.sp().set(Sp::_0));
                }
            }

            fn wait_for_stop(&self) -> Result<(), I2cError> {
                unsafe{ 
                    let mut timeout = 0;
                    while self.iic.icsr2().read().stop().get().0 == 0 {
                        if timeout > 10000 {
                            return Err(I2cError::PeripheralNotStopped);
                        }
                        timeout += 1;
                    }
                }
                Ok(())
            }

            fn clear_stop_flag(&mut self) {
                unsafe {
                    self.iic.icsr2().modify(|w| w.stop().set(Stop::_0));
                }
            }

            fn transmit_buffer_ready(&self) -> bool {
                unsafe {
                    self.iic.icsr2().read().tdre().get().0 == 1
                }
            }

            fn wait_transmit_buffer_ready(&self) -> Result<(), I2cError> {
                let mut timeout = 0;
                while !self.transmit_buffer_ready() {
                    if timeout > 10000 {
                        return Err(I2cError::TransmitBufferNotReady);
                    }
                    timeout += 1;
                }
                Ok(())
            }

            fn is_data_received(&self) -> bool {
                unsafe {
                    self.iic.icsr2().read().rdrf().get().0 == 1
                }
            }

            fn wait_data_received(&self) -> Result<(), I2cError> {
                let mut timeout = 0;
                while !self.is_data_received() {
                    if timeout > 10000 {
                        return Err(I2cError::DataNotReceived);
                    }
                    timeout += 1;
                }
                Ok(())
            }

            fn slave_acknowledged(&self) -> bool {
                unsafe {
                    self.iic.icsr2().read().nackf().get().0 == 0
                }
            }

            fn clear_nack_flag(&mut self) {
                unsafe {
                    self.iic.icsr2().modify(|w| w.nackf().set(Nackf::_0));
                }
            }

            fn transmit_complete(&self) -> bool {
                unsafe {
                    self.iic.icsr2().read().tend().get().0 == 1
                }
            }

            fn wait_transmit_complete(&self) -> Result<(), I2cError> {
                let mut timeout = 0;
                while !self.transmit_complete() {
                    if timeout > 10000 {
                        return Err(I2cError::SlaveNotResponding);
                    }
                    timeout += 1;
                }
                Ok(())
            }

            fn set_wait(&mut self) {
                unsafe {
                    self.iic.icmr3().modify(|w| w.wait().set(Wait::_1));
                }
            }

            fn clear_wait(&mut self) {
                unsafe {
                    self.iic.icmr3().modify(|w| w.wait().set(Wait::_0));
                }
            }

            fn add_byte_to_transmit(&mut self, byte: u8) {
                unsafe {
                    self.iic.icdrt().modify(|w| w.set_raw(byte));
                }
            }

            fn read_byte(&mut self) -> u8 {
                unsafe {
                    self.iic.icdrr().read().get_raw()
                }
            }

            fn acknowledge_slave(&mut self) {
                unsafe {
                    // Turn off write protect first
                    self.iic.icmr3().modify(|w| w.ackwp().set(Ackwp::_1));
                    // Acknowledge the slave
                    self.iic.icmr3().modify(|w| w.ackbt().set(Ackbt::_1));
                }
            }

            /// Writes data to the I2C slave device at the specified address.
            pub fn write(&mut self, address: u8, data: &[u8]) -> Result<(), I2cError> {
                self.initialize_settings(address);

                self.wait_for_bus()?;

                self.start();
                
                let mut acknowledgement_issue = false;
                let bytes_to_transmit = data.len() + 1; // +1 for the address byte
                let mut bytes_transmitted = 0;

                while bytes_transmitted < bytes_to_transmit {
                    if self.slave_acknowledged() {
                        if self.transmit_buffer_ready() {
                            // transmit data
                            match bytes_transmitted {
                                0 => {
                                    self.add_byte_to_transmit(get_slave_address(address, Direction::Write));
                                }
                                _ => {
                                    self.add_byte_to_transmit(data[bytes_transmitted - 1]);
                                }
                            }
                            bytes_transmitted += 1;
                        }
                    }else{
                        acknowledgement_issue = true;
                        break;
                    }
                }

                if acknowledgement_issue == false {
                    self.wait_transmit_complete()?;
                }

                self.clear_stop_flag();
                
                self.stop();
                
                self.wait_for_stop()?;

                self.clear_nack_flag();

                self.unstop();


                if acknowledgement_issue {
                    return Err(I2cError::SlaveNotResponding);
                }

                Ok(())
            }

            pub fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), I2cError> {
                self.initialize_settings(address);

                self.wait_for_bus()?;

                self.start();

                self.wait_transmit_buffer_ready()?;

                self.add_byte_to_transmit(get_slave_address(address, Direction::Read));

                self.wait_data_received()?;

                if self.slave_acknowledged() == false {
                    self.clear_stop_flag();
                    self.stop();
                    self.read_byte(); // dummy read
                    self.wait_for_stop()?;
                    self.clear_nack_flag();
                    self.clear_stop_flag();
                    return Err(I2cError::SlaveNotResponding);
                }

                self.read_byte(); // dummy read 

                for i in 0..buffer.len() {
                    self.wait_data_received()?;

                    if i == buffer.len() - 1 {
                        // Last byte has some extra stuff to do
                        break;
                    }else if i == buffer.len() - 2 {
                        // Second to last byte, we need to acknowledge the slave
                        self.set_wait();
                    }

                    buffer[i] = self.read_byte();
                }

                self.acknowledge_slave();

                buffer[buffer.len() - 1] = self.read_byte();

                self.wait_data_received()?;

                self.clear_stop_flag();

                self.stop();

                self.read_byte(); // dummy read

                self.clear_wait();

                self.wait_for_stop()?;

                self.clear_nack_flag();

                self.clear_stop_flag();

                Ok(())
            }
        }

        impl<SDA: I2cSDAPin, SCL: I2cSCLPin> embedded_hal::blocking::i2c::Write<SevenBitAddress> for I2c0<SDA, SCL> {

            type Error = I2cError;

            fn write(&mut self, address: u8, write: &[u8]) -> Result<(), Self::Error> {
                self.write(address, write)
            }
        }

        impl<SDA: I2cSDAPin, SCL: I2cSCLPin> embedded_hal::blocking::i2c::Read<SevenBitAddress> for I2c0<SDA, SCL> {
            type Error = I2cError;

            fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
                self.read(address, buffer)
            }
        }

        impl<SDA: I2cSDAPin, SCL: I2cSCLPin> embedded_hal::blocking::i2c::WriteRead<SevenBitAddress> for I2c0<SDA, SCL> {
            type Error = I2cError;

            fn write_read(&mut self, address: u8, write: &[u8], read: &mut [u8]) -> Result<(), Self::Error> {
                self.write(address, write)?;
                self.read(address, read)
            }
        }
    }
}
#[cfg(feature = "iic0")]
define_i2c!(I2c0, Iic0, enable_i2c0);

#[cfg(feature = "iic1")]   
define_i2c!(I2c1, Iic1, enable_i2c1);
