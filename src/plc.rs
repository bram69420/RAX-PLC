use crate::error::PlcError;
use crate::ffi;

pub struct Plc {
    // no flag state, lifetime struct dah ckup
}

impl Plc {
    pub fn connect() -> Result<Self, PlcError> {
        let result = unsafe { ffi::plc_init() };
        if result != 0 {
            return Err(PlcError::from(result));
        }
        Ok(Self {})
    }

    pub fn read_u32(&self, address: u32) -> Result<u32, PlcError> {
        let mut value = 0u32;
        let result = unsafe { ffi::plc_read_u32(address, &mut value) };
        if result != 0 {
            return Err(PlcError::from(result));
        }
        Ok(value)
    }

    pub fn write_u32(&self, address: u32, value: u32) -> Result<(), PlcError> {
        let result = unsafe { ffi::plc_write_u32(address, value) };
        if result != 0 {
            return Err(PlcError::from(result));
        }
        Ok(())
    }
}

impl Drop for Plc {
    fn drop(&mut self) {
        unsafe {
            let _ = ffi::plc_shutdown();
        }
    }
}