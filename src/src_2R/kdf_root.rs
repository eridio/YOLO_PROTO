
use hkdf::Hkdf;

#[cfg(feature = "ring")]
use ring_compat::digest::Sha512;

#[cfg(not(feature = "ring"))]
use sha2::Sha512;

use core::convert::TryInto;

use p256::ecdh::SharedSecret;

pub fn kdf_rk(rk: &[u8; 32], dh_out: &SharedSecret) -> ([u8; 32], [u8; 32]) {
    let h = Hkdf::<Sha512>::new(Some(rk), dh_out.as_bytes());
    let mut okm = [0u8; 64];
    let info = b"Root Key Info";
    h.expand(info, &mut okm).unwrap();
    let (a, b) = okm.split_at(32);
    (a.try_into()
         .expect("Incorrect length"),
     b.try_into()
         .expect("Incorrect length"))
}

pub fn kdf_rk_he(rk: &[u8; 32], dh_out: &SharedSecret) -> ([u8; 32], [u8; 32], [u8; 32]) {
    let h = Hkdf::<Sha512>::new(Some(rk), dh_out.as_bytes());
    let mut okm = [0u8; 96];
    let info = b"Root Key Generator";
    h.expand(info, &mut okm).unwrap();
    let (rk, a) = okm.split_at(32);
    let (ck, nhk) = a.split_at(32);
    (
        rk.try_into().expect("Wrong length"),
        ck.try_into().expect("Wrong length"),
        nhk.try_into().expect("Wrong length")
    )
}
