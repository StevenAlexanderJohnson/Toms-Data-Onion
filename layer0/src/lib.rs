type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn decode_ascii85(input: String) -> Result<Vec<u8>> {
    return ascii85::decode(&input).map_err(|e| e.into());
}
