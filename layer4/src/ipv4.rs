use std::net::Ipv4Addr;
use crate::udp::validate_udp_data;
use crate::utils::{ChecksumResult, calculate_checksum};

#[derive(Debug)]
pub struct IPv4Header {
    pub total_length: u16,
    pub source_address: Ipv4Addr,
    pub destination_address: Ipv4Addr,

    pub data: Option<Vec<u8>>,
}

impl IPv4Header {
    pub fn from(bytes: &[u8]) -> ChecksumResult<Self> {
        let ihl = bytes[0] & 0b00001111;
        let header_length = (ihl * 4) as usize;

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
            total_length,
            source_address,
            destination_address,
            data: validate_udp_data(&data, source_address, destination_address),
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