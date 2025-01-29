#![no_std]

use embedded_hal::i2c::I2c;
use error::Error;
use registers::{
    MfrCalibration, MfrId, MfrModel, MfrRevision, ReadIin, ReadVin, RestoreDefaultAll, TiMfrId,
    TiMfrModel, TiMfrRevision,
};
use uom::si::f32::{ElectricCurrent, ElectricPotential, ElectricalResistance};

pub mod error;

mod bits;
mod registers;

/// A driver for a Texas Instruments INA233.
///
/// Generic over the device's I2C bus `S` and address `A`.
pub struct Ina233<S: I2c, const A: u8> {
    i2c: S,

    current_lsb: ElectricCurrent,
}

impl<S: I2c, const A: u8> Ina233<S, A> {
    /// Construct a new driver instance.
    ///
    /// Will error on a mismatch in any of the component identification
    /// registers, unless the `no-verify` feature is active. Resets the device
    /// before continuing.
    ///
    /// - `shunt_resistance`: The resistance of the current shunt.
    /// - `maximum_current`: The maximum expected (measurable) current load.
    pub fn new(
        i2c: S,
        shunt_resistance: ElectricalResistance,
        maximum_current: ElectricCurrent,
    ) -> Result<Self, Error<S>> {
        let mut ina233 = Self {
            i2c,
            current_lsb: maximum_current / 0x8000 as f32,
        };

        #[cfg(not(feature = "no-verify"))]
        {
            let mfr_id = unsafe { ina233.read_register::<MfrId, 3>()? };
            let mfr_model = unsafe { ina233.read_register::<MfrModel, 7>()? };
            let mfr_revision = unsafe { ina233.read_register::<MfrRevision, 3>()? };
            let ti_mfr_id = unsafe { ina233.read_register::<TiMfrId, 2>()? };
            let ti_mfr_model = unsafe { ina233.read_register::<TiMfrModel, 2>()? };
            let ti_mfr_revision = unsafe { ina233.read_register::<TiMfrRevision, 2>()? };

            if mfr_id.id() != "TI"
                || mfr_model.model() != "INA233"
                || mfr_revision.revision() != "A1"
                || ti_mfr_id.id() != "TI"
                || ti_mfr_model.model() != "33"
                || ti_mfr_revision.revision() != "A1"
            {
                return Err(Error::Verification);
            }
        }

        unsafe { ina233.write_register(RestoreDefaultAll)? };

        let mut mfr_calibration = MfrCalibration::default();
        mfr_calibration.set_current_lsb(ina233.current_lsb, shunt_resistance);
        unsafe { ina233.write_register(mfr_calibration)? };

        Ok(ina233)
    }

    /// Read the latest voltage measurement.
    pub fn voltage(&mut self) -> Result<ElectricPotential, Error<S>> {
        Ok(unsafe { self.read_register::<ReadVin, 2>()? }.voltage())
    }

    /// Read the latest current measurement.
    pub fn current(&mut self) -> Result<ElectricCurrent, Error<S>> {
        Ok(unsafe { self.read_register::<ReadIin, 2>()? }.current(self.current_lsb))
    }
}
