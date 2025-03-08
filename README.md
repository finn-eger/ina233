A Rust driver for the [Texas Instruments INA233][Texas Instruments] power monitor.
_Currently incomplete, with support for basic (continuous) readings only._

For usage details and explanatory notes, see the [documentation][Docs.rs].

### Overview

```rust
let i2c: Bus = todo!(/* Setup I2C */);

// Setup a device:
const ADDRESS: u8 = 0x40;
let shunt_resistance = ElectricalResistance::new::<milliohm>(5.0);
let maximum_current = ElectricCurrent::new::<ampere>(1.0);
let mut device = Ina233::<_, ADDRESS>::new(i2c, shunt_resistance, maximum_current)?;

// Get latest readings:
device.voltage()?;
device.current()?;
```

[Texas Instruments]: https://www.ti.com/product/INA233
[Docs.rs]: https://docs.rs/ina233/latest
