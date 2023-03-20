#[cfg(feature = "image-bitshift")]
/// Do a lot of convoluted things involving the image crate to
/// come up with a checksum value. Don't use this for anything,
/// it's horribly slow and not at all useful.
pub mod image_bitshift;

#[cfg(feature = "cosmological_constants")]
/// Cosmological Constants are entirely a figment of Chat-GPTs imagination
pub mod cosmological_constants;
