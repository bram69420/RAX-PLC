use plc_connector::{Plc, PlcError};

fn setup_plc() -> Plc {
    Plc::connect().expect("Gagal terhubung ke PLC")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_then_read() {
        let plc = setup_plc();
        let target_value = 0x12345678;
        let address = 0;

        plc.write_u32(address, target_value).unwrap();
        let value = plc.read_u32(address).unwrap();
        
        assert_eq!(value, target_value, "Nilai yang dibaca tidak sesuai dengan yang ditulis");
    }

    #[test]
    fn test_multiple_addresses() {
        let plc = setup_plc();
        let test_cases = vec![(0, 100), (4, 200), (8, 300)];

        for &(addr, val) in &test_cases {
            plc.write_u32(addr, val).unwrap();
        }
        //overlap
        for &(addr, val) in &test_cases {
            assert_eq!(plc.read_u32(addr).unwrap(), val);
        }
    }

    #[test]
    fn test_invalid_address() {
        let plc = setup_plc();
        let invalid_address = 4096; //4095 mem
        
        let result = plc.read_u32(invalid_address);
        
        assert!(result.is_err(), "Harusnya mengembalikan error untuk alamat di luar jangkauan");
        assert_eq!(result.unwrap_err(), PlcError::InvalidAddress);
    }

    #[test]
    fn test_boundary_values() {
        let plc = setup_plc();
        let address = 16;

        plc.write_u32(address, u32::MIN).unwrap();
        assert_eq!(plc.read_u32(address).unwrap(), u32::MIN);

        plc.write_u32(address, u32::MAX).unwrap();
        assert_eq!(plc.read_u32(address).unwrap(), u32::MAX);
    }
}
