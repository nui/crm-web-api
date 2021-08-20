use ring::signature::{Ed25519KeyPair, UnparsedPublicKey, ED25519};

pub struct PrivateKey(Ed25519KeyPair);

impl PrivateKey {
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        Ed25519KeyPair::from_seed_unchecked(bytes).map(Self).ok()
    }

    pub fn sign(&self, msg: &[u8]) -> Vec<u8> {
        self.0.sign(msg).as_ref().to_vec()
    }

    pub fn from_base64<T: ?Sized + AsRef<[u8]>>(input: &T) -> Option<Self> {
        base64::decode_config(input, base64::URL_SAFE_NO_PAD)
            .ok()
            .and_then(|seed| Self::from_bytes(&seed))
    }
}

#[derive(Clone)]
pub struct PublicKey(UnparsedPublicKey<Vec<u8>>);

impl PublicKey {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self(UnparsedPublicKey::new(&ED25519, bytes.to_vec()))
    }

    pub fn from_base64<T: ?Sized + AsRef<[u8]>>(input: &T) -> Option<Self> {
        base64::decode_config(input, base64::URL_SAFE_NO_PAD)
            .map(|bytes| Self(UnparsedPublicKey::new(&ED25519, bytes)))
            .ok()
    }

    pub fn verify(&self, message: &[u8], signature: &[u8]) -> bool {
        self.0.verify(message, signature).is_ok()
    }
}

#[cfg(test)]
pub mod tests {
    pub fn get_test_public_key() -> String {
        String::from("y9OTFvZmHe41kMjCYtDd8574bv46CSDKexUKN9R7mgM")
    }

    pub fn get_test_private_key() -> String {
        String::from("aMWX1G0p36BRx7YqAJaBJ7hnMDxqIbln0toRQcWQfoA")
    }
}
