use std::fmt;

#[allow(dead_code)]
enum Error {
    OK = 0x00,
    IllegalFunction = 0x01,     // The function code received in the query is not an allowable action for the server.
    IllegalDataAddress = 0x02,  // The data address received in the query is not an allowable address for the server.
    IllegalDataValue = 0x03,    // A value contained in the query data field is not an allowable value for server.
    ServerDeviceFailure = 0x04, // An unrecoverable error occurred while the server w
}

#[derive(PartialEq)]
pub enum PDUError {
    //ErrorNo,
    ErrorLength,
    ErrorCRC,
}

impl fmt::Debug for PDUError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            PDUError::ErrorNo => "PDUError(ErrorNo)",
            PDUError::ErrorLength => "PDUError(ErrorLength)",
            PDUError::ErrorCRC => "PDUError(ErrorCRC)",
        };
        write!(f, "({})", msg)
    }
}

pub fn check_frame(buff: Vec<u8>) -> Result<(), PDUError> {
    if buff.len() < 6 {
        return Err(PDUError::ErrorLength);
    }
    let (hi_crc, lo_crc) = crc16(buff[0..buff.len() - 2].to_vec());
    if (hi_crc != buff[buff.len() - 2]) || (lo_crc != buff[buff.len() - 1]) {
        return Err(PDUError::ErrorCRC);
    }
    Ok(())
}

fn crc16(buff: Vec<u8>) -> (u8, u8) {
    let mut crc: u16 = 0xFFFF; // 16 bit crc preset register

    for byte in buff {
        let temp = byte as u16; // the eight data registers or will crc
        crc ^= temp; // store data register crc
        for _ in 0..8 {
            // loop calculation data
            let bit = crc & 0x01 == 0x01;
            crc >>= 1; // first data right one
            if bit {
                // determines that is not the right one, if one is XORed with the polynomial.
                crc ^= 0xA001; // with the above exclusive-or polynomial
            }
        }
    }
    ((crc & 0xFF) as u8, (crc >> 8) as u8)
}
