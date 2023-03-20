pub fn triple_hash<T: AsRef<str>>(input: T) -> u64 {
    let mut result = input
        .as_ref()
        .chars()
        .rev()
        .map(|c| c as u64)
        .fold(0u64, |acc, x| (acc << 1) ^ (x >> 1));

    let mut divisor = result;
    for i in 0..3 {
        result = ((result << 1) ^ (result >> (64 - (i as u64 + 1).next_power_of_two()))) % divisor;
        divisor /= 3;
    }
    result
}
