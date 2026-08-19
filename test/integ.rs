use plc_connector::{Plc, PlcError};

#[test]
fn write_then_read() {
    let plc = Plc::connect().unwrap();

    plc.write_u32(0, 0x12345678).unwrap();

    let value = plc.read_u32(0).unwrap();

    assert_eq!(value, 0x12345678);
}

#[test]
fn multiple_addresses() {
    let plc = Plc::connect().unwrap();

    plc.write_u32(0, 100).unwrap();
    plc.write_u32(4, 200).unwrap();
    plc.write_u32(8, 300).unwrap();

    assert_eq!(plc.read_u32(0).unwrap(), 100);
    assert_eq!(plc.read_u32(4).unwrap(), 200);
    assert_eq!(plc.read_u32(8).unwrap(), 300);
}

#[test]
fn invalid_address() {
    let plc = Plc::connect().unwrap();

    let result = plc.read_u32(4096);

    assert_eq!(result, Err(PlcError::InvalidAddress));
}
