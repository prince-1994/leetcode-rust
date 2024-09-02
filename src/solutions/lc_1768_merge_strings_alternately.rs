pub fn merge_alternately(word1: String, word2: String) -> String {
    let mut result = String::new();
    let n = word1.len() + word2.len();
    let (mut word1, mut word2) = (word1.chars(), word2.chars());

    for _ in 0..n {
        if let Some(c) = word1.next() {
            result.push(c);
        }
        if let Some(c) = word2.next() {
            result.push(c);
        }
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            merge_alternately("abc".to_string(), "pqr".to_string()),
            "apbqcr".to_string()
        );
        assert_eq!(
            merge_alternately("ab".to_string(), "pqrs".to_string()),
            "apbqrs".to_string()
        );
        assert_eq!(
            merge_alternately("abcd".to_string(), "pq".to_string()),
            "apbqcd".to_string()
        );
    }
}
