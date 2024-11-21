use std::net::Ipv4Addr;
use crate::utils::calculate_checksum;

#[derive(Debug)]
struct UDPPseudoHeader {
    pub source_address: Ipv4Addr,
    pub destination_address: Ipv4Addr,
    pub zeroes: u8,
    pub protocol: u8,
    pub udp_length: u16,
}

impl UDPPseudoHeader {
    fn new(source_address: Ipv4Addr, destination_address: Ipv4Addr, udp_length: u16) -> Self {
        Self {
            source_address,
            destination_address,
            zeroes: 0u8,
            protocol: 0x11,
            udp_length,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut output = vec![];
        output.extend_from_slice(&self.destination_address.octets());
        output.extend_from_slice(&self.source_address.octets());
        output.extend_from_slice(&self.zeroes.to_be_bytes());
        output.extend_from_slice(&self.protocol.to_be_bytes());
        output.extend_from_slice(&self.udp_length.to_be_bytes());

        output
    }
}

pub fn validate_udp_data(bytes: &[u8], source_address: Ipv4Addr, destination_address: Ipv4Addr) -> Option<Vec<u8>> {
    let source_port = (u16::from(bytes[0]) << 8) | bytes[1] as u16;
    let destination_port = (u16::from(bytes[2]) << 8) | bytes[3] as u16;
    let length = (u16::from(bytes[4]) << 8) | bytes[5] as u16;
    let expected_checksum = u16::from(bytes[6]) << 8 | u16::from(bytes[7]);
    let data = Vec::from(&bytes[8..]);
    if destination_port != 42069 {
        return None;
    }

    let mut checksum_bytes = Vec::<u8>::new();
    checksum_bytes.extend_from_slice(&UDPPseudoHeader::new(source_address, destination_address, length).to_bytes());
    checksum_bytes.extend_from_slice(&source_port.to_be_bytes());
    checksum_bytes.extend_from_slice(&destination_port.to_be_bytes());
    checksum_bytes.extend_from_slice(&length.to_be_bytes());
    checksum_bytes.extend_from_slice(&data);
    if checksum_bytes.len() % 2 != 0 {
        checksum_bytes.push(0x0);
    }

    if calculate_checksum(&checksum_bytes) != expected_checksum {
        return None;
    }

    Some(data)
}