# WIP - RA4M2 Rust HAL

First attempt at a Rust HAL for the RA4M2 series microcontroller. Pretty bare bones right now:
- I2C Read and Write (Only Port4 pins work) 
- GPIO (Port4 only)
- embedded_time and half working embassy_time_driver
- Interrupt registration and clearing
- Peripheral power control
- HOCO and Main Clock oscillator control

Added an I2C example using an MPU6050.

