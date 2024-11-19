type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn decode(input: &[u8]) -> Result<Vec<u8>> {
    let output = input
        .iter()
        .map(|&x| {
            let byte = x ^ 0b01010101;
            let carry = (byte & 1) << 7;
            (byte >> 1) | carry
        })
        .collect();
    Ok(output)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_decode() {
        let input = vec![0b00000000, 0b10110100];
        let output = super::decode(&input).unwrap();
        assert_eq!(output, vec![0b10101010, 0b11110000]);
    }
}
