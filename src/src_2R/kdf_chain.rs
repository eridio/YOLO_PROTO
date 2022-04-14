use hmac::{Hmac, Mac, NewMac};

#[cfg(feature = "ring")]
use ring_compat::digest::Sha512;

#[cfg(not(feature = "ring"))]
use sha2::Sha512;

use core::convert::TryInto;

type HmacSha512 = Hmac<Sha512>;

pub fn kdf_ck(ck: &[u8; 32]) -> ([u8; 32], [u8; 32]) {
    let mac = HmacSha512::new_from_slice(ck)
        .expect("Invalid Key Length");
    let result = mac.finalize().into_bytes();
    let (a, b) = result.split_at(32);
    (a.try_into()
        .expect("Incorrect Length"),
    b.try_into()
        .expect("Incorrect Length"))
}
