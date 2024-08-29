pub fn length_of_last_word(s: String) -> i32 {
    let val = match s.trim().split_whitespace().last() {
        Some(word) => word.len() as i32,
        None => 0,
    };
    return val;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn test() {
        assert_eq!(length_of_last_word(" Hello    World ".to_string()), 5);
        assert_eq!(length_of_last_word("Hello World".to_string()), 5);
        assert_eq!(length_of_last_word("Hello".to_string()), 5);
    }
}
