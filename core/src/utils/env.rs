/// Gets the number of rows which by default should be used for each chip to maximize padding.
///
/// Some chips, such as FieldLTU, may use a constant multiple of this value to optimize performance.
pub fn shard_size() -> usize {
    let value = match std::env::var("SHARD_SIZE") {
        Ok(val) => val.parse().unwrap(),
        Err(_) => 1 << 20,
    };
    assert!(value != 0 && (value & (value - 1)) == 0);
    value
}