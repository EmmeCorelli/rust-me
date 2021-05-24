use modbus::PDUError;

fn main() {
    let buffer = vec![0x012, 0x06, 0x1a, 0x1a, 0x12, 0x34, 0xa2, 0x62];
    let result = match modbus::check_frame(buffer) {
        Ok(()) => PDUError::ErrorNo,
        Err(e) => e,
    };
    println!("{:?}", result)
}
