mod ipv4;
mod udp;
use crate::ipv4::IPv4Header;

pub fn decrypt(input: &[u8]) -> Vec<u8> {
    let mut offset = 0;
    let mut payloads = vec![];
    let mut invalid_paylaods = vec![];
    while offset < input.len() {
        let payload = match IPv4Header::from(&input[offset..]) {
            Ok(x) => x,
            Err(e) => {
                offset += e.total_length as usize;
                invalid_paylaods.push(e);
                continue;
            }
        };
        offset += payload.total_length as usize;

        payloads.push(payload)
    }

    for payload in payloads.iter() {
        println!("{:?}", payload);
    }

    vec![]
}