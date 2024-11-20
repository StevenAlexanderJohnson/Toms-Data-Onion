mod ipv4;
mod udp;
mod utils;

use crate::ipv4::IPv4Header;

pub fn get_data(input: &[u8]) -> Vec<u8> {
    let mut offset = 0;
    let mut payloads = vec![];
    let mut invalid_payloads = vec![];
    while offset < input.len() {
        let payload = match IPv4Header::from(&input[offset..]) {
            Ok(x) => x,
            Err(e) => {
                offset += e.total_length as usize;
                invalid_payloads.push(e);
                continue;
            }
        };
        offset += payload.total_length as usize;

        payloads.push(payload)
    }

    let output = payloads.iter().flat_map(|packet| packet.data.iter().flatten()).cloned().collect::<Vec<_>>();

    output
}