use std::net::Ipv4Addr;
use crate::udp::validate_udp_data;
use crate::utils::calculate_checksum;

// The u16 is to represent the total size of the IPv4 packet so lib knows how much to skip for the
// next packet.
type IPv4ParseError<T> = Result<T, u16>;

#[derive(Debug)]
pub struct IPv4Header {
    pub total_length: u16,
    pub data: Option<Vec<u8>>,
}

impl IPv4Header {
    pub fn from(bytes: &[u8]) -> IPv4ParseError<Self> {
        let ihl = bytes[0] & 0b00001111;
        let header_length = (ihl * 4) as usize;

        let total_length = (u16::from(bytes[2]) << 8) | bytes[3] as u16;

        let mut checksum_slice = Vec::with_capacity(bytes.len() - 2);
        checksum_slice.extend_from_slice(&bytes[0..10]);
        checksum_slice.extend_from_slice(&bytes[12..header_length as usize]);

        let source_address = Ipv4Addr::from_bits(u32::from_be_bytes([
            bytes[12], bytes[13], bytes[14], bytes[15],
        ]));
        let destination_address = Ipv4Addr::from_bits(u32::from_be_bytes([
            bytes[16], bytes[17], bytes[18], bytes[19],
        ]));

        let header_checksum = (u16::from(bytes[10]) << 8) | bytes[11] as u16;
        let expected_checksum = calculate_checksum(&checksum_slice);

        let data = validate_udp_data(&bytes[header_length..total_length as usize], source_address, destination_address);

        if expected_checksum != header_checksum
            || source_address != Ipv4Addr::new(10, 1, 1, 10)
            || destination_address != Ipv4Addr::new(10, 1, 1, 200)
            || output.data.is_none()
        {
            return Err(total_length);
        }

        Ok(Self {
            total_length,
            data,
        })
    }
}