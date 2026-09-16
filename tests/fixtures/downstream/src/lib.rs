#![no_std]

use okerrr::okerrr;

pub struct Opaque;

pub fn ordinary(input: Result<u8, Opaque>) -> u8 {
    okerrr!(input, 0)
}

#[cfg(feature = "with_tracing")]
pub fn caller_traced(input: Result<u8, &'static str>) -> u8 {
    okerrr!(input, error => {
        tracing::error!(error = ?error, "caller handled error");
        0
    })
}
