#[cfg(feature = "equality-hashers")]
/// A collection of hash algorithms that always has as themselves. This is
/// actively useless.
pub mod equality_hashers;

#[cfg(feature = "tripple-string-hash")]
/// Do something called a tripple hash. This probably doesn't do anything
/// at all useful, so don't use it.
pub mod tripple_string_hash;
