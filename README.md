# WIP - RA4M2 Rust HAL

First attempt at a Rust HAL for the RA4M2 series microcontroller. Pretty bare bones right now:
- I2C Read and Write
- GPIO on ports 0-7 (feature-gated: `port0` through `port7`; `port4` is on by default)
- embedded_time and half working embassy_time_driver
- Interrupt registration and clearing
- Peripheral power control
- HOCO and Main Clock oscillator control

Examples:
- `examples/i2c` — I2C reads from an MPU6050, LEDs on port 4
- `examples/gpio` — toggles pins across ports 0-7 (skips the SWD/JTAG, boot-mode, and oscillator pins)

## Known GPIO limitations / follow-ups / punting to future work

Only I2C and GPIO are implemented today, so none of these are a problem yet:

- Need to add CI w/ clippy or a commit-hook, currently missing. 
- `into_alternate_function` hardcodes open-drain + low drive. Correct for I2C, 
  wrong for push-pull peripherals (SCI TX, GPT PWM, CAN). Parameterize the 
  drain/drive settings before adding any of those drivers.
- Pin function changes write PSEL and PMR in a single register write. The
  hardware manual's procedure is: clear PMR, write PSEL, then set PMR. Only
  matters when re-muxing a pin that is already in peripheral mode; 
  config from reset is fine.
- The PAC reuses one register-block type per port group (`Port1` for ports 1-4,
  `Port0` for ports 0/5/6/7, and the tokens are `Copy`), so `PortN::new` cannot
  verify it was handed the matching `PORTn` token, and constructing a port twice
  compiles. Pass the right token, once. The proper fix is upstream in ra-pac to
  emit a distinct non-`Copy` type per port instance, resulting in a compile error.
- The low-level `pfsel::portN` functions silently ignore pin numbers that don't
  exist on the port. The typed `Pin` API can't reach that path; direct callers
  must pass valid pins.
- Pin reads/writes go through the PmnPFS registers under a critical section and
  the PFS write-protect feature. The PORT blocks' PODR/POSR/PORR registers would
  allow lock-free atomic pin writes; switching `set_pin_value`/`get_pin_value`
  over to them is the correct long term path.
- Use traits to ensure correctness of pin types w/ alternate functions. This is   
  currently controlled by codegen, but should be in the type system.  
