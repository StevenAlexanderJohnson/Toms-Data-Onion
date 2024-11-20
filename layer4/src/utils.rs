pub fn calculate_checksum(headers: &[u8]) -> u16 {
    let mut sum = headers
        .chunks(2)
        .map(|bytes| {
            let word = if bytes.len() == 1 {
                u16::from_be_bytes([bytes[0], 0])
            } else {
                u16::from_be_bytes([bytes[0], bytes[1]])
            };
            u32::from(word)
        })
        .fold(0u32, |acc, word| {
            let sum = acc + word;
            let output = if sum >> 16 != 0 {
                (sum & 0xFFFF) + (sum >> 16)
            } else {
                sum
            };
            output
        });
    while (sum >> 16) != 0 {
        sum = (sum & 0xFFFF) + (sum >> 16);
    }
    !(sum) as u16
}
