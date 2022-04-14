//! Implementation of the double ratchet system/encryption as specified by [Signal][1].
//!
//! **WARNING! This implementation uses P-256 NOT Curve25519 as specified by Signal!**
//!
//! The implementation follows the cryptographic recommendations provided by [Signal][2].
//! The AEAD Algorithm uses a constant Nonce. This might be changed in the future.
//!
//! # Example Usage:
//!
//! ## Standard:
//! ```
//! use double_ratchet_2::ratchet::Ratchet;
//!
//! let sk = [1; 32];                                                 // Initial Key created by a symmetric key agreement protocol
//! let (mut bob_ratchet, public_key) = Ratchet::init_bob(sk);        // Creating Bobs Ratchet (returns Bobs PublicKey)
//! let mut alice_ratchet = Ratchet::init_alice(sk, public_key);      // Creating Alice Ratchet with Bobs PublicKey
//! let data = b"Hello World".to_vec();                               // Data to be encrypted
//! let ad = b"Associated Data";                                      // Associated Data
//!
//! let (header, encrypted, nonce) = alice_ratchet.ratchet_encrypt(&data, ad);   // Encrypting message with Alice Ratchet (Alice always needs to send the first message)
//! let decrypted = bob_ratchet.ratchet_decrypt(&header, &encrypted, &nonce, ad); // Decrypt message with Bobs Ratchet
//! assert_eq!(data, decrypted)
//! ```
//!
//! ## With lost message:
//! ```
//! # use double_ratchet_2::ratchet::Ratchet;
//!
//! let sk = [1; 32];                                                 // Initial Key created by a symmetric key agreement protocol
//! let (mut bob_ratchet, public_key) = Ratchet::init_bob(sk);        // Creating Bobs Ratchet (returns Bobs PublicKey)
//! let mut alice_ratchet = Ratchet::init_alice(sk, public_key);      // Creating Alice Ratchet with Bobs PublicKey
//! let data = b"Hello World".to_vec();                               // Data to be encrypted
//! let ad = b"Associated Data";                                      // Associated Data
//!
//! let (header1, encrypted1, nonce1) = alice_ratchet.ratchet_encrypt(&data, ad); // Lost message
//! let (header2, encrypted2, nonce2) = alice_ratchet.ratchet_encrypt(&data, ad); // Successful message
//!
//! let decrypted2 = bob_ratchet.ratchet_decrypt(&header2, &encrypted2, &nonce2, ad); // Decrypting second message first
//! let decrypted1 = bob_ratchet.ratchet_decrypt(&header1, &encrypted1, &nonce1, ad); // Decrypting latter message
//!
//! let comp = decrypted1 == data && decrypted2 == data;
//! assert!(comp);
//! ```
//!
//! ## Encryption before recieving inital message
//!
//! ```should_panic
//! use double_ratchet_2::ratchet::Ratchet;
//! let sk = [1; 32];
//! let ad = b"Associated Data";
//! let (mut bob_ratchet, _) = Ratchet::init_bob(sk);
//! let data = b"Hello World".to_vec();
//!
//! let (_, _, _) = bob_ratchet.ratchet_encrypt(&data, ad);
//! ```
//!
//! ## Encryption after recieving initial message
//! However bob can (of course) also encrypt messages. This is possible, after decrypting the first message from alice.
//!
//! ```
//! use double_ratchet_2::ratchet::Ratchet;
//! let sk = [1; 32];
//!
//! let (mut bob_ratchet, public_key) = Ratchet::init_bob(sk);
//! let mut alice_ratchet = Ratchet::init_alice(sk, public_key);
//!
//! let data = b"Hello World".to_vec();
//! let ad = b"Associated Data";
//!
//! let (header1, encrypted1, nonce1) = alice_ratchet.ratchet_encrypt(&data, ad);
//! let _decrypted1 = bob_ratchet.ratchet_decrypt(&header1, &encrypted1, &nonce1, ad);
//!
//! let (header2, encrypted2, nonce2) = bob_ratchet.ratchet_encrypt(&data, ad);
//! let decrypted2 = alice_ratchet.ratchet_decrypt(&header2, &encrypted2, &nonce2, ad);
//!
//! assert_eq!(data, decrypted2);
//! ```
//! ## Constructing and Deconstructing Headers
//!
//! ```
//! # use double_ratchet_2::ratchet::Ratchet;
//! # use double_ratchet_2::header::Header;
//! # let sk = [1; 32];
//! # let (mut bob_ratchet, public_key) = Ratchet::init_bob(sk);
//! # let mut alice_ratchet = Ratchet::init_alice(sk, public_key);
//! # let data = b"hello World".to_vec();
//! # let ad = b"Associated Data";
//! # let (header, _, _) = alice_ratchet.ratchet_encrypt(&data, ad);
//! let header_bytes: Vec<u8> = header.clone().into();
//! let header_const = Header::from(header_bytes);
//! assert_eq!(header, header_const);
//! ```
//!
//! # Example Ratchet with encrypted headers
//!
//! ```
//! use double_ratchet_2::ratchet::RatchetEncHeader;
//! let sk = [0; 32];
//! let shared_hka = [1; 32];
//! let shared_nhkb = [2; 32];
//!
//! let (mut bob_ratchet, public_key) = RatchetEncHeader::init_bob(sk, shared_hka, shared_nhkb);
//! let mut alice_ratchet = RatchetEncHeader::init_alice(sk, public_key, shared_hka, shared_nhkb);
//! let data = b"Hello World".to_vec();
//! let ad = b"Associated Data";
//!
//! let (header, encrypted, nonce) = alice_ratchet.ratchet_encrypt(&data, ad);
//! let decrypted = bob_ratchet.ratchet_decrypt(&header, &encrypted, &nonce, ad);
//! assert_eq!(data, decrypted)
//! ```
//!
//! # Export / Import Ratchet with encrypted headers
//! This ratchet implements import and export functionality. This works over a bincode backend and
//! maybe useful for saving Ratchets to and loading from a file.
//! ```
//! # use double_ratchet_2::ratchet::RatchetEncHeader;
//! # let sk = [0; 32];
//! # let shared_hka = [1; 32];
//! # let shared_nhkb = [2; 32];
//! let (bob_ratchet, public_key) = RatchetEncHeader::init_bob(sk, shared_hka, shared_nhkb);
//! let ex_ratchet = bob_ratchet.export();
//! let im_ratchet = RatchetEncHeader::import(&ex_ratchet);
//! assert_eq!(im_ratchet, bob_ratchet)
//! ```
//!
//! # Features
//!
//! Currently the crate only supports one feature: ring. If feature is enabled the crate switches
//! to ring-compat and uses ring as backend for Sha512 Hashing. May result in slightly better performance.
//!
//!
//! TODO:
//! - [x] Standard Double Ratchet
//! - [x] [Double Ratchet with encrypted headers][3]
//!
//! [1]: https://signal.org/docs/specifications/doubleratchet/
//! [2]: https://signal.org/docs/specifications/doubleratchet/#recommended-cryptographic-algorithms
//! [3]: https://signal.org/docs/specifications/doubleratchet/#double-ratchet-with-header-encryption

#![no_std]
#![allow(stable_features)]

extern crate alloc;

pub use p256::PublicKey;

mod aead;
pub mod dh;
mod kdf_root;
mod kdf_chain;

pub mod ratchet;

/// Message Header
pub mod header;

