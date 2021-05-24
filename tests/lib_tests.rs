use modbus;

fn assert_modbus(buff: Vec<u8>, expected: modbus::PDUError) {
    let result = match modbus::check_frame(buff) {
        Ok(()) => modbus::PDUError::ErrorNo,
        Err(e) => e,
    };
    assert_eq!(result, expected);
}

#[test]
fn short_message() {
    let buffer = vec![0x00];
    assert_modbus(buffer, modbus::PDUError::ErrorLength);
}

#[test]
fn check_crc_of_null_fails() {
    let buffer = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    assert_modbus(buffer, modbus::PDUError::ErrorCRC);
}

#[test]
fn check_crc_of_null() {
    let buffer = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1B];
    assert_modbus(buffer, modbus::PDUError::ErrorNo);
}

#[test]
fn check_crc_message() {
    let buffer = vec![0x01, 0x06, 0x1a, 0x1a, 0x12, 0x34, 0xa2, 0x62];
    assert_modbus(buffer, modbus::PDUError::ErrorNo);
}
