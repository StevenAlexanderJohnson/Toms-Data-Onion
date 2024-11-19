type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn decode(input: &[u8]) -> Result<Vec<u8>> {
    // Filter the input bytes so that only the bytes with a valid parity bit are kept.
    // Then drop the parity bit.
    let payload: Vec<u8> = input
        .iter()
        .filter(|&b| b.count_ones() % 2 == 0)
        .copied()
        .collect();

    Ok(payload
        .chunks(8)
        .flat_map(|chunk| {
            let mut buffer = 0u64;
            for &byte in chunk {
                // Clearing the parity bit, can also (byte >> 1) << 1
                let byte = byte & 0xFE;
                buffer |= byte as u64;
                buffer <<= 7;
            }

            let take_number = 7;
            buffer
                .to_be_bytes()
                .iter()
                .take(take_number)
                .cloned()
                .collect::<Vec<u8>>()
        })
        .collect())
}