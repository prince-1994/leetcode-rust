pub fn reverse_bits(n: u32) -> u32 {
    let mut result = 0;
    for i in 0..32 {
        result |= ((n >> i) & 1) << (31 - i);
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(reverse_bits(43261596), 964176192);
        assert_eq!(reverse_bits(4294967293), 3221225471);
    }
}
