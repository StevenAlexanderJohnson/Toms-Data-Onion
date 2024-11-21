use openssl::{
    aes::{aes_ige, unwrap_key, AesKey},
    symm::Mode,
};

pub fn decrypt(input: &[u8]) -> Vec<u8> {
    let kek: [u8; 32] = input[0..32].try_into().unwrap();
    let kek_iv: [u8; 8] = input[32..40].try_into().unwrap();
    let wrapped_key: [u8; 40] = input[40..80].try_into().unwrap();
    let payload_iv: [u8; 16] = input[80..96].try_into().unwrap();
    let mut ige_iv = [0u8; 32];
    ige_iv[..16].copy_from_slice(&payload_iv);
    ige_iv[16..].copy_from_slice(&payload_iv);
    let mut payload: Vec<u8> = input[96..].to_vec();

    let kek = AesKey::new_decrypt(&kek).expect("Unable to create AesKey from KEK");
    let mut key = [0u8; 32];
    unwrap_key(&kek, Some(kek_iv), &mut key, &wrapped_key).expect("Unable to unwrap the key");

    let key = AesKey::new_decrypt(&key).expect("Unable to create AesKey from key");

    // The length of the payload has to be a multiple of 16
    let padding_length = (16 - (payload.len() % 16)) % 16;
    if padding_length > 0 {
        payload.resize(payload.len() + padding_length, 0);
    }

    let mut output = vec![0; payload.len()];
    aes_ige(&payload, &mut output, &key, &mut ige_iv, Mode::Decrypt);

    output
}
