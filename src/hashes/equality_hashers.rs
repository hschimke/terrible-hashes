use std::any::Any;

#[cfg(feature = "pure-equal")]
pub fn pure_equality_hash<T: Any + Clone>(data: &T) -> T {
    data.clone()
}
