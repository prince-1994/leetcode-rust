use std::cmp::max;
use std::collections::HashSet;
pub fn length_of_longest_substring(s: String) -> usize {
    let mut result = 0;
    let mut set = HashSet::new();
    let mut start = 0;
    let chars: Vec<char> = s.chars().collect();
    for i in 0..chars.len() {
        let c = chars[i];
        while set.contains(&c) {
            set.remove(&chars[start]);
            start += 1;
        }
        set.insert(c);
        result = max(result, i - start + 1);
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
        assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
        assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
    }
}
