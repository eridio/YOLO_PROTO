use rand_core::OsRng;
use core::fmt::{Debug, Formatter};
use core::fmt;
use p256::PublicKey as PublicKey;
use p256::ecdh::SharedSecret;
use p256::SecretKey;
use p256::elliptic_curve::ecdh::diffie_hellman;
use serde::Serialize;
use zeroize::Zeroize;

#[derive(Clone)]
pub struct DhKeyPair {
    pub private_key: SecretKey,
    pub public_key: PublicKey,
}

impl Drop for DhKeyPair {
    fn drop(&mut self) {
        self.private_key = SecretKey::random(&mut OsRng);
        self.public_key = self.private_key.public_key();
    }
}

impl Zeroize for DhKeyPair {
    fn zeroize(&mut self) {
        self.private_key = SecretKey::random(&mut OsRng);
        self.public_key = self.private_key.public_key();
    }
}

impl DhKeyPair {
    pub fn ex_public_key_bytes(&self) -> Vec<u8> {
        self.public_key.to_string().as_bytes().to_vec()
    }
    pub fn ex_private_key_bytes(&self) -> Vec<u8> {
        self.private_key.to_bytes().to_vec()
    }
    pub fn from_bytes(priv_key : Vec<u8> , pub_key: String) -> DhKeyPair{
        DhKeyPair {
            private_key : SecretKey::from_bytes(priv_key).unwrap(),
            public_key : PublicKey::from_jwk_str(&pub_key).unwrap(),
        }
    }
}

impl PartialEq for DhKeyPair {
    fn eq(&self, other: &Self) -> bool {
        if self.private_key.to_bytes() != other.private_key.to_bytes() {
            return false
        }
        if self.ex_public_key_bytes() != other.ex_public_key_bytes() {
            return false
        }
        true
    }
}

impl Debug for DhKeyPair {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("DhKeyPair")
            .field("private_key", &self.private_key.to_bytes())
            .field("public_key", &self.ex_public_key_bytes())
            .finish()
    }
}

impl Default for DhKeyPair {
    fn default() -> Self {
        Self::new()
    }
}

impl DhKeyPair {
    pub fn new() -> Self {
        let secret = SecretKey::random(&mut OsRng);
        let public = secret.public_key();
        DhKeyPair {
            private_key: secret,
            public_key: public,
        }
    }

    pub fn key_agreement(&self, public_key: &PublicKey) -> SharedSecret {
        diffie_hellman(self.private_key.to_secret_scalar(), public_key.as_affine())
    }
}
