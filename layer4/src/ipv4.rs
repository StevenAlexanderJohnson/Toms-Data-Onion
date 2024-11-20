use std::net::Ipv4Addr;
use crate::udp::UDPHeader;
use crate::utils::{ChecksumResult, calculate_checksum};


#[derive(Debug)]
pub struct IPv4Header {
    pub version: u8,
    pub ihl: u8,
    pub dscp: u8,
    pub ecn: u8,
    pub total_length: u16,
    pub identification: u16,
    pub flags: u8,
    pub fragment_offset: u16,
    pub ttl: u8,
    pub protocol: u8,
    pub header_checksum: u16,
    pub source_address: Ipv4Addr,
    pub destination_address: Ipv4Addr,

    // Options
    pub options: Vec<u8>,

    pub data: Option<UDPHeader>,
}

impl IPv4Header {
    pub fn from(bytes: &[u8]) -> ChecksumResult<Self> {
        let ihl = bytes[0] & 0b00001111;
        let header_length = (ihl * 4) as usize;
        let options = Vec::from(&bytes[20..header_length]);

        let total_length = (u16::from(bytes[2]) << 8) | bytes[3] as u16;
        let data: Vec<u8> = Vec::from(&bytes[header_length..total_length as usize]);

        let mut checksum_slice = Vec::with_capacity(bytes.len() - 2);
        checksum_slice.extend_from_slice(&bytes[0..10]);
        checksum_slice.extend_from_slice(&bytes[12..header_length as usize]);

        let header_checksum = (u16::from(bytes[10]) << 8) | bytes[11] as u16;
        let expected_checksum = calculate_checksum(&checksum_slice);

        let source_address = Ipv4Addr::from_bits(u32::from_be_bytes([
            bytes[12], bytes[13], bytes[14], bytes[15],
        ]));
        let destination_address= Ipv4Addr::from_bits(u32::from_be_bytes([
            bytes[16], bytes[17], bytes[18], bytes[19],
        ]));

        let output = Self {
            version: bytes[0] >> 4,
            ihl,
            dscp: bytes[1] >> 2,
            ecn: bytes[1] & 0b00000011,
            total_length,
            identification: (u16::from(bytes[4]) << 8) | bytes[5] as u16,
            flags: bytes[6] >> 5,
            fragment_offset: (u16::from(bytes[6] & 0b00011111) << 5) | bytes[7] as u16,
            ttl: bytes[8],
            protocol: bytes[9],
            header_checksum,
            source_address,
            destination_address,
            options,
            data: UDPHeader::from(&data, source_address, destination_address),
        };

        if expected_checksum != header_checksum
            || output.source_address != Ipv4Addr::new(10, 1, 1, 10)
            || output.destination_address != Ipv4Addr::new(10, 1, 1, 200)
            || output.data.is_none()
        {
            return Err(output);
        }

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let bytes = [
            0x45, 0x00, 0x00, 0x3c, 0x1c, 0x46, 0x40, 0x00, 0x40, 0x06, 0xac, 0x10, 0x0a, 0x63,
            0xac, 0x10, 0x0a, 0x0c,
        ];

        let checksum = calculate_checksum(&bytes);

        assert_eq!(checksum, 0b1011000111100110);
    }

    #[test]
    fn test_checksum() {
        let bytes = [
            69, 0, 0, 29, 0, 0, 64, 0, 58, 17, 41, 253, 10, 1, 1, 10, 10, 1, 1, 200,
        ];

        let mut checksum_bytes = Vec::with_capacity(&bytes.len() - 2);
        checksum_bytes.extend_from_slice(&bytes[0..10]);
        checksum_bytes.extend_from_slice(&bytes[12..]);

        let checksum = calculate_checksum(&checksum_bytes);

        assert_eq!(checksum, 10749);
    }
}