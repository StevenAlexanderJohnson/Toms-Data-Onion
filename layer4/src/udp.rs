#[derive(Debug)]
pub struct UDPHeader {
    source_port: u16,
    destination_port: u16,
    length: u16,
    checksum: u16,
    data: Vec<u8>,
}


impl UDPHeader {
    pub fn from(bytes: &[u8]) -> Self {
        Self {
            source_port: (u16::from(bytes[0]) << 8) | bytes[1] as u16,
            destination_port: (u16::from(bytes[2]) << 8) | bytes[3] as u16,
            length: (u16::from(bytes[4]) << 8) | bytes[5] as u16,
            checksum: (u16::from(bytes[6]) << 8) | bytes[7] as u16,
            data: Vec::from(&bytes[8..]),
        }
    }
}