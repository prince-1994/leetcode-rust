pub fn str_str(haystack: String, needle: String) -> i32 {
    if let Some(index) = haystack.find(&needle) {
        return index as i32;
    }
    return -1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn test() {
        assert_eq!(str_str("hello".to_string(), "llo".to_string()), 2);
        assert_eq!(str_str("aaaaa".to_string(), "bba".to_string()), -1);
    }
}
