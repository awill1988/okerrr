#![no_std]

use okerrr::okerrr;

pub struct Opaque;

pub fn ordinary(input: Result<u8, Opaque>) -> Result<u8, Opaque> {
    let value = okerrr!(input, case error => return Err(error));
    Ok(value)
}

#[cfg(feature = "with_tracing")]
pub fn caller_traced(input: Result<u8, &'static str>) -> Result<u8, &'static str> {
    let value = okerrr!(input, case error => {
        tracing::error!(error = ?error, "caller handled error");
        return Err(error);
    });
    Ok(value)
}
