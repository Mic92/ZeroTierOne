
mod error;
mod cipher_ctx;
mod bn;
mod ec;

pub mod aes_gmac_siv;
pub mod secret;
pub mod random;
pub mod aes;
pub mod hash;
pub mod mimcvdf;
pub mod p384;

pub mod poly1305;
pub mod salsa;
pub mod typestate;
pub mod x25519;


/// This must be called before using any function from this library.
pub fn init() {
    ffi::init();
}

/// Constant time byte slice equality.
#[inline]
pub fn secure_eq<A: AsRef<[u8]> + ?Sized, B: AsRef<[u8]> + ?Sized>(a: &A, b: &B) -> bool {
    let (a, b) = (a.as_ref(), b.as_ref());
    if a.len() == b.len() {
        let mut x = 0u8;
        for (aa, bb) in a.iter().zip(b.iter()) {
            x |= *aa ^ *bb;
        }
        x == 0
    } else {
        false
    }
}
