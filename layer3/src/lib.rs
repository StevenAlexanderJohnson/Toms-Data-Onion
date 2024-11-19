// I'm going to put an explination here because I'm pretty proud of this one.
// The challenge was to decrypt the payload which was encrypted with a block cipher.
// You were not given the key and only told that the key was 32 bytes long.

// I looked at the old layer inputs and found that there was always a line of dashes to separate the sections of the input.
// I used that line and applied it to chunks of 32 bytes in the encrypted message to find all possible keys.
// I then used each of those keys and applied it to the input file, decrypt originally returned Vec<Vec<u8>>.
// I printed each of those byte arrays to an output file and found one that was mostly decrypted.
// I messed up the find_possible_keys function because I didn't take 32 bytes from the expected string, so I found one that was mostly decrypted but had issues.

// I was able to determine what the first line should say so I created find_key.
// I took took the first 32 bytes from the expected string and applied it to the first 32 bytes of the encrypted message to get the key and decrypted the rest of the file
// using that key.

fn apply_block(input: &[u8], block: &[u8]) -> Vec<u8> {
    input
        .iter()
        .zip(block.iter().cycle())
        .map(|(a, b)| a ^ b)
        .collect()
}

#[allow(dead_code)]
fn find_possible_keys(input: &[u8]) -> Vec<Vec<u8>> {
    let key = "    ----------------------------".as_bytes().to_vec();
    input
        .chunks(32)
        .map(|block| apply_block(block, &key))
        .collect()
}

fn find_key(input: &[u8]) -> Vec<u8> {
    // Collected from layer3_49.txt
    let expected_text = "==[ Layer 4/6: Network Traffic ]============================="
        .as_bytes()
        .chunks(32)
        .next()
        .unwrap();

    let key = input.chunks(32).next().unwrap();
    apply_block(expected_text, key)
}

pub fn decrypt(input: &[u8]) -> Vec<u8> {
    let key = find_key(input);
    apply_block(input, &key)
    // let possible_keys = find_possible_keys(input);
    // possible_keys
    //     .iter()
    //     .map(|key| apply_block(input, key))
    //     .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_block() {
        let input = [0; 32];
        let block = [1; 32];
        let output = apply_block(&input, &block);
        assert_eq!(output, block);
    }

    #[test]
    fn test_example() {
        let input: [u8; 7] = [0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77];
        let block: [u8; 3] = [0xAA, 0xBB, 0xCC];
        let output = apply_block(&input, &block);
        assert_eq!(output, [0xBB, 0x99, 0xFF, 0xEE, 0xEE, 0xAA, 0xDD]);
    }
}
