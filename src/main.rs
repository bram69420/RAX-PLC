use plc_connector::Plc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let plc = Plc::connect()?;

    println!("PLC connected");

    plc.write_u32(0, 1234)?;
    println!("Wrote 1234 to PLC address 0");

    let value = plc.read_u32(0)?;
    println!("Read {} from PLC address 0", value);

    plc.write_u32(4, 5678)?;

    let value = plc.read_u32(4)?;
    println!("Read {} from PLC address 4", value);

    Ok(())
}
