#![allow(unused)]

use core::str;

use embedded_hal::i2c::{I2c, SevenBitAddress};
use uom::si::electric_current::ampere;
use uom::si::electric_potential::volt;
use uom::si::f32::{ElectricCurrent, ElectricPotential, ElectricalResistance};
use uom::si::ratio::ratio;

use crate::bits::GetBits;
use crate::Ina233;

const CLEAR_FAULTS: u8 = 0x03;
const RESTORE_DEFAULT_ALL: u8 = 0x12;
const CAPABILITY: u8 = 0x19;
const IOUT_OC_WARN_LIMIT: u8 = 0x4A;
const VIN_OV_WARN_LIMIT: u8 = 0x57;
const VIN_UV_WARN_LIMIT: u8 = 0x58;
const PIN_OP_WARN_LIMIT: u8 = 0x6B;
const STATUS_BYTE: u8 = 0x78;
const STATUS_WORD: u8 = 0x79;
const STATUS_IOUT: u8 = 0x7B;
const STATUS_INPUT: u8 = 0x7C;
const STATUS_CML: u8 = 0x7E;
const STATUS_MFR_SPECIFIC: u8 = 0x80;
const READ_EIN: u8 = 0x86;
const READ_VIN: u8 = 0x88;
const READ_IIN: u8 = 0x89;
const READ_VOUT: u8 = 0x8B;
const READ_IOUT: u8 = 0x8C;
const READ_POUT: u8 = 0x96;
const READ_PIN: u8 = 0x97;
const MFR_ID: u8 = 0x99;
const MFR_MODEL: u8 = 0x9A;
const MFR_REVISION: u8 = 0x9B;
const MFR_ADC_CONFIG: u8 = 0xD0;
const MFR_READ_VSHUNT: u8 = 0xD1;
const MFR_ALERT_MASK: u8 = 0xD2;
const MFR_CALIBRATION: u8 = 0xD4;
const MFR_DEVICE_CONFIG: u8 = 0xD5;
const CLEAR_EIN: u8 = 0xD6;
const TI_MFR_ID: u8 = 0xE0;
const TI_MFR_MODEL: u8 = 0xE1;
const TI_MFR_REVISION: u8 = 0xE2;

pub(crate) struct ClearFaults;

impl From<ClearFaults> for [u8; 1] {
    fn from(_register: ClearFaults) -> Self {
        [ClearFaults::ADDRESS]
    }
}

impl WritableRegister<1> for ClearFaults {
    const ADDRESS: u8 = CLEAR_FAULTS;
}

pub(crate) struct RestoreDefaultAll;

impl From<RestoreDefaultAll> for [u8; 1] {
    fn from(_register: RestoreDefaultAll) -> Self {
        [RestoreDefaultAll::ADDRESS]
    }
}

impl WritableRegister<1> for RestoreDefaultAll {
    const ADDRESS: u8 = RESTORE_DEFAULT_ALL;
}

pub(crate) struct IoutOcWarnLimit {
    value: u16,
}

impl From<[u8; 2]> for IoutOcWarnLimit {
    fn from(bytes: [u8; 2]) -> Self {
        Self {
            value: bytes.bits::<1, 12>(),
        }
    }
}

impl ReadableRegister<2> for IoutOcWarnLimit {
    const ADDRESS: u8 = IOUT_OC_WARN_LIMIT;
}

pub(crate) struct VinOvWarnLimit {
    value: u16,
}

impl From<[u8; 2]> for VinOvWarnLimit {
    fn from(bytes: [u8; 2]) -> Self {
        Self {
            value: bytes.bits::<1, 12>(),
        }
    }
}

impl ReadableRegister<2> for VinOvWarnLimit {
    const ADDRESS: u8 = VIN_OV_WARN_LIMIT;
}

pub(crate) struct VinUvWarnLimit {
    value: u16,
}

impl From<[u8; 2]> for VinUvWarnLimit {
    fn from(bytes: [u8; 2]) -> Self {
        Self {
            value: bytes.bits::<1, 12>(),
        }
    }
}

impl ReadableRegister<2> for VinUvWarnLimit {
    const ADDRESS: u8 = VIN_UV_WARN_LIMIT;
}

pub(crate) struct PinOpWarnLimit {
    value: u16,
}

impl From<[u8; 2]> for PinOpWarnLimit {
    fn from(bytes: [u8; 2]) -> Self {
        Self {
            value: bytes.bits::<4, 12>(),
        }
    }
}

impl ReadableRegister<2> for PinOpWarnLimit {
    const ADDRESS: u8 = PIN_OP_WARN_LIMIT;
}

pub(crate) struct StatusByte {
    cml: bool,
    none_of_the_above: bool,
}

impl From<[u8; 1]> for StatusByte {
    fn from(bytes: [u8; 1]) -> Self {
        Self {
            cml: bytes.bit::<1>(),
            none_of_the_above: bytes.bit::<0>(),
        }
    }
}

impl ReadableRegister<1> for StatusByte {
    const ADDRESS: u8 = STATUS_BYTE;
}

pub(crate) struct StatusWord {
    iout_pout: bool,
    input: bool,
    mfr: bool,
    cml: bool,
    none_of_the_above: bool,
}

impl From<[u8; 2]> for StatusWord {
    fn from(bytes: [u8; 2]) -> Self {
        Self {
            iout_pout: bytes.bit::<14>(),
            input: bytes.bit::<13>(),
            mfr: bytes.bit::<12>(),
            cml: bytes.bit::<1>(),
            none_of_the_above: bytes.bit::<0>(),
        }
    }
}

impl ReadableRegister<2> for StatusWord {
    const ADDRESS: u8 = STATUS_WORD;
}

pub(crate) struct StatusIout {
    iout_oc_warn: bool,
}

impl From<[u8; 1]> for StatusIout {
    fn from(bytes: [u8; 1]) -> Self {
        Self {
            iout_oc_warn: bytes.bit::<5>(),
        }
    }
}

impl ReadableRegister<1> for StatusIout {
    const ADDRESS: u8 = STATUS_IOUT;
}

pub(crate) struct StatusInput {
    vin_ov_warn: bool,
    vin_uv_warn: bool,
    iin_oc_warn: bool,
    pin_op_warn: bool,
}

impl From<[u8; 1]> for StatusInput {
    fn from(bytes: [u8; 1]) -> Self {
        Self {
            vin_ov_warn: bytes.bit::<6>(),
            vin_uv_warn: bytes.bit::<5>(),
            iin_oc_warn: bytes.bit::<1>(),
            pin_op_warn: bytes.bit::<0>(),
        }
    }
}

impl ReadableRegister<1> for StatusInput {
    const ADDRESS: u8 = STATUS_INPUT;
}

pub(crate) struct StatusCml {
    invalid_or_unsupported_command_recieved: bool,
    packet_error_check_failed: bool,
    memory_fault_detected: bool,
}

impl From<[u8; 1]> for StatusCml {
    fn from(bytes: [u8; 1]) -> Self {
        Self {
            invalid_or_unsupported_command_recieved: bytes.bit::<14>(),
            packet_error_check_failed: bytes.bit::<5>(),
            memory_fault_detected: bytes.bit::<1>(),
        }
    }
}

impl ReadableRegister<1> for StatusCml {
    const ADDRESS: u8 = STATUS_CML;
}

pub(crate) struct StatusMfrSpecific {
    conversion_ready: bool,
    arithmetic_overflow_flag: bool,
    power_on_reset_event_detected: bool,
    communications_or_memory_fault: bool,
    input_overpower_warning: bool,
    input_overcurrent_warning: bool,
    input_overvoltage_warning: bool,
    input_undervoltage_warning: bool,
}

impl From<[u8; 1]> for StatusMfrSpecific {
    fn from(bytes: [u8; 1]) -> Self {
        Self {
            conversion_ready: bytes.bit::<7>(),
            arithmetic_overflow_flag: bytes.bit::<6>(),
            power_on_reset_event_detected: bytes.bit::<5>(),
            communications_or_memory_fault: bytes.bit::<4>(),
            input_overpower_warning: bytes.bit::<3>(),
            input_overcurrent_warning: bytes.bit::<2>(),
            input_overvoltage_warning: bytes.bit::<1>(),
            input_undervoltage_warning: bytes.bit::<0>(),
        }
    }
}

impl ReadableRegister<1> for StatusMfrSpecific {
    const ADDRESS: u8 = STATUS_MFR_SPECIFIC;
}

pub(crate) struct ReadEin {
    sample_count: u32,
    power_accumulator_rollover_count: u8,
    power_accumulator: u16,
}

impl From<[u8; 7]> for ReadEin {
    fn from(bytes: [u8; 7]) -> Self {
        Self {
            sample_count: u32::from_le_bytes([bytes[4], bytes[5], bytes[6], 0x00]),
            power_accumulator_rollover_count: bytes[3],
            power_accumulator: u16::from_le_bytes([bytes[1], bytes[2]]),
        }
    }
}

impl ReadableRegister<7> for ReadEin {
    const ADDRESS: u8 = READ_EIN;
}

pub(crate) struct ReadVin {
    value: u16,
}

impl ReadVin {
    pub(crate) fn voltage(&self) -> ElectricPotential {
        convert_to_voltage(self.value)
    }
}

impl From<[u8; 2]> for ReadVin {
    fn from(bytes: [u8; 2]) -> Self {
        Self {
            value: bytes.bits::<0, 16>(),
        }
    }
}

impl ReadableRegister<2> for ReadVin {
    const ADDRESS: u8 = READ_VIN;
}

pub(crate) struct ReadIin {
    value: i16,
}

impl ReadIin {
    pub(crate) fn current(&self, current_lsb: ElectricCurrent) -> ElectricCurrent {
        convert_to_current(self.value, current_lsb)
    }
}

impl From<[u8; 2]> for ReadIin {
    fn from(bytes: [u8; 2]) -> Self {
        Self {
            value: bytes.bits::<0, 16>() as i16,
        }
    }
}

impl ReadableRegister<2> for ReadIin {
    const ADDRESS: u8 = READ_IIN;
}

pub(crate) struct MfrAdcConfig {
    avg: u8,
    vbusct: u8,
    vshct: u8,
    mode: u8,
}

impl From<[u8; 2]> for MfrAdcConfig {
    fn from(bytes: [u8; 2]) -> Self {
        Self {
            avg: bytes.bits::<9, 3>() as u8,
            vbusct: bytes.bits::<6, 3>() as u8,
            vshct: bytes.bits::<3, 3>() as u8,
            mode: bytes.bits::<0, 3>() as u8,
        }
    }
}

impl ReadableRegister<2> for MfrAdcConfig {
    const ADDRESS: u8 = MFR_ADC_CONFIG;
}

pub(crate) struct MfrId {
    id: [u8; 2],
}

impl MfrId {
    pub(crate) fn id(&self) -> &str {
        str::from_utf8(&self.id).unwrap()
    }
}

impl From<[u8; 3]> for MfrId {
    fn from(bytes: [u8; 3]) -> Self {
        Self {
            id: [bytes[1], bytes[2]],
        }
    }
}

impl ReadableRegister<3> for MfrId {
    const ADDRESS: u8 = MFR_ID;
}

pub(crate) struct MfrModel {
    model: [u8; 6],
}

impl MfrModel {
    pub(crate) fn model(&self) -> &str {
        str::from_utf8(&self.model).unwrap()
    }
}

impl From<[u8; 7]> for MfrModel {
    fn from(bytes: [u8; 7]) -> Self {
        Self {
            model: [bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6]],
        }
    }
}

impl ReadableRegister<7> for MfrModel {
    const ADDRESS: u8 = MFR_MODEL;
}

pub(crate) struct MfrRevision {
    revision: [u8; 2],
}

impl MfrRevision {
    pub(crate) fn revision(&self) -> &str {
        str::from_utf8(&self.revision).unwrap()
    }
}

impl From<[u8; 3]> for MfrRevision {
    fn from(bytes: [u8; 3]) -> Self {
        Self {
            revision: [bytes[1], bytes[2]],
        }
    }
}

impl ReadableRegister<3> for MfrRevision {
    const ADDRESS: u8 = MFR_REVISION;
}

pub(crate) struct MfrReadVshunt {
    value: i16,
}

impl From<[u8; 2]> for MfrReadVshunt {
    fn from(bytes: [u8; 2]) -> Self {
        Self {
            value: bytes.bits::<0, 16>() as i16,
        }
    }
}

impl ReadableRegister<2> for MfrReadVshunt {
    const ADDRESS: u8 = MFR_READ_VSHUNT;
}

pub(crate) struct MfrAlertMask {
    conversion_ready: bool,
    adc_overflow_detected: bool,
    por_event_detected: bool,
    communications: bool,
    in_op_warning: bool,
    in_oc_warning: bool,
    in_ov_warning: bool,
    in_uv_warning: bool,
}

impl From<[u8; 1]> for MfrAlertMask {
    fn from(bytes: [u8; 1]) -> Self {
        Self {
            conversion_ready: bytes.bit::<7>(),
            adc_overflow_detected: bytes.bit::<6>(),
            por_event_detected: bytes.bit::<5>(),
            communications: bytes.bit::<4>(),
            in_op_warning: bytes.bit::<3>(),
            in_oc_warning: bytes.bit::<2>(),
            in_ov_warning: bytes.bit::<1>(),
            in_uv_warning: bytes.bit::<0>(),
        }
    }
}

impl ReadableRegister<1> for MfrAlertMask {
    const ADDRESS: u8 = MFR_ALERT_MASK;
}

#[derive(Default)]
pub(crate) struct MfrCalibration {
    value: u16,
}

impl MfrCalibration {
    pub(crate) fn current_lsb(
        &mut self,
        shunt_resistance: ElectricalResistance,
    ) -> ElectricCurrent {
        ElectricPotential::new::<volt>(0.00512) / (self.value as f32 * shunt_resistance)
    }

    pub(crate) fn set_current_lsb(
        &mut self,
        current_lsb: ElectricCurrent,
        shunt_resistance: ElectricalResistance,
    ) {
        // NOTE: If this value clips, subsequent reads of the in-memory register will be incorrect.
        self.value = (ElectricPotential::new::<volt>(0.00512) / (current_lsb * shunt_resistance))
            .get::<ratio>() as u16;
    }
}

impl From<[u8; 2]> for MfrCalibration {
    fn from(bytes: [u8; 2]) -> Self {
        Self {
            value: u16::from_le_bytes(bytes),
        }
    }
}

impl From<MfrCalibration> for [u8; 3] {
    fn from(register: MfrCalibration) -> Self {
        let bytes = register.value.to_le_bytes();
        [
            <MfrCalibration as WritableRegister<3>>::ADDRESS,
            bytes[0],
            bytes[1],
        ]
    }
}

impl ReadableRegister<2> for MfrCalibration {
    const ADDRESS: u8 = MFR_CALIBRATION;
}

impl WritableRegister<3> for MfrCalibration {
    const ADDRESS: u8 = MFR_CALIBRATION;
}

pub(crate) struct MfrDeviceConfig {
    ein_status: bool,
    ein_accum: u8,
    i2c_filt: bool,
    read_ein_autoclear: bool,
    alert_behavior: bool,
    apol: bool,
}

impl From<[u8; 1]> for MfrDeviceConfig {
    fn from(bytes: [u8; 1]) -> Self {
        Self {
            ein_status: bytes.bit::<7>(),
            ein_accum: bytes.bits::<4, 2>(),
            i2c_filt: bytes.bit::<3>(),
            read_ein_autoclear: bytes.bit::<2>(),
            alert_behavior: bytes.bit::<1>(),
            apol: bytes.bit::<0>(),
        }
    }
}

impl ReadableRegister<1> for MfrDeviceConfig {
    const ADDRESS: u8 = MFR_DEVICE_CONFIG;
}

pub(crate) struct ClearEin;

impl From<ClearEin> for [u8; 1] {
    fn from(_register: ClearEin) -> Self {
        [ClearEin::ADDRESS]
    }
}

impl WritableRegister<1> for ClearEin {
    const ADDRESS: u8 = CLEAR_EIN;
}

pub(crate) struct TiMfrId {
    id: [u8; 2],
}

impl TiMfrId {
    pub(crate) fn id(&self) -> &str {
        str::from_utf8(&self.id).unwrap()
    }
}

impl From<[u8; 2]> for TiMfrId {
    fn from(mut bytes: [u8; 2]) -> Self {
        bytes.reverse();
        Self { id: bytes }
    }
}

impl ReadableRegister<2> for TiMfrId {
    const ADDRESS: u8 = TI_MFR_ID;
}

pub(crate) struct TiMfrModel {
    model: [u8; 2],
}

impl TiMfrModel {
    pub(crate) fn model(&self) -> &str {
        str::from_utf8(&self.model).unwrap()
    }
}

impl From<[u8; 2]> for TiMfrModel {
    fn from(mut bytes: [u8; 2]) -> Self {
        bytes.reverse();
        Self { model: bytes }
    }
}

impl ReadableRegister<2> for TiMfrModel {
    const ADDRESS: u8 = TI_MFR_MODEL;
}

pub(crate) struct TiMfrRevision {
    revision: [u8; 2],
}

impl TiMfrRevision {
    pub(crate) fn revision(&self) -> &str {
        str::from_utf8(&self.revision).unwrap()
    }
}

impl From<[u8; 2]> for TiMfrRevision {
    fn from(mut bytes: [u8; 2]) -> Self {
        bytes.reverse();
        Self { revision: bytes }
    }
}

impl ReadableRegister<2> for TiMfrRevision {
    const ADDRESS: u8 = TI_MFR_REVISION;
}

pub(crate) trait ReadableRegister<const L: usize>: From<[u8; L]> {
    const ADDRESS: u8;
}

pub(crate) trait WritableRegister<const L: usize>: Into<[u8; L]> {
    const ADDRESS: u8;
}

impl<S: I2c, const A: u8> Ina233<S, A> {
    // TODO: Try to find a way to avoid having to specify the length generic every time I call these methods.
    pub(crate) unsafe fn read_register<R: ReadableRegister<L>, const L: usize>(
        &mut self,
    ) -> Result<R, S::Error> {
        let mut bytes = [0x00; L];
        self.i2c.write_read(A, &[R::ADDRESS], &mut bytes)?;
        Ok(bytes.into())
    }

    pub(crate) unsafe fn write_register<R: WritableRegister<L>, const L: usize>(
        &mut self,
        register: R,
    ) -> Result<(), S::Error> {
        // FIXME: In the future (when arithmetic with const generics is stable)
        // this should be rewritten to separate register addressing from values.
        self.i2c.write(A, &register.into())
    }
}

// TODO: It may be possible to do these conversions with better precision.
fn convert_to_voltage(raw: u16) -> ElectricPotential {
    ElectricPotential::new::<volt>(raw as f32 * 0.001_25)
}

fn convert_to_shunt_voltage(raw: i16) -> ElectricPotential {
    ElectricPotential::new::<volt>(raw as f32 * 0.000_002_5)
}

fn convert_to_current(raw: i16, current_lsb: ElectricCurrent) -> ElectricCurrent {
    raw as f32 * current_lsb
}
