use openssl::{
    aes::{unwrap_key, AesKey}, symm
};

pub fn decrypt(input: &[u8]) -> Vec<u8> {
    let kek: [u8; 32] = input[0..32].try_into().unwrap();
    let kek_iv: [u8; 8] = input[32..40].try_into().unwrap();
    let wrapped_key: [u8; 40] = input[40..80].try_into().unwrap();
    let payload_iv: [u8; 16] = input[80..96].try_into().unwrap();
    let mut payload: Vec<u8> = input[96..].to_vec();

    let kek = AesKey::new_decrypt(&kek).expect("Unable to create AesKey from KEK");
    let mut key = [0u8; 32];
    unwrap_key(&kek, Some(kek_iv), &mut key, &wrapped_key).expect("Unable to unwrap the key");

    // The length of the payload has to be a multiple of 16
    let padding_length = (16 - (payload.len() % 16)) % 16;
    if padding_length > 0 {
        payload.resize(payload.len() + padding_length, 0);
    }

    let cipher = symm::Cipher::aes_256_ctr();
    symm::decrypt(cipher, &key, Some(&payload_iv), &payload).expect("Unable to decrypt message")
}
