use std::num::NonZeroU32;

use rand::rngs::OsRng;
use rand::RngCore;
use ring::pbkdf2::{derive, PBKDF2_HMAC_SHA256};
use ring::signature::{Ed25519KeyPair, KeyPair};

fn main() {
    let mut secret = [0u8; 32];
    let mut salt = [0u8; 32];

    // Random secret and salt
    OsRng.fill_bytes(&mut secret);
    OsRng.fill_bytes(&mut salt);

    let mut count = 0;
    loop {
        let mut key = [0u8; 32];
        derive(
            PBKDF2_HMAC_SHA256,
            NonZeroU32::new(100_000 + count).unwrap(),
            &salt,
            &secret,
            &mut key,
        );
        let key_pair = Ed25519KeyPair::from_seed_unchecked(&key).unwrap();
        let private_key = base64::encode_config(&key, base64::URL_SAFE_NO_PAD);
        let public_key =
            base64::encode_config(key_pair.public_key().as_ref(), base64::URL_SAFE_NO_PAD);
        count += 1;

        let is_alphanum = |s: &str| s.chars().all(char::is_alphanumeric);
        // Only accept alphanumeric keys for ease of coping
        if is_alphanum(&private_key) && is_alphanum(&public_key) {
            println!("Public : {}\n", &public_key);
            println!("SECRET : {}\n", &private_key);
            break;
        }
    }
}
