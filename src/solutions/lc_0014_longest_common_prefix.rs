use std::cmp::min;

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    fn common_prefix(s1: &str, s2: &str) -> String {
        let mut result = String::new();
        let mut i = 0;
        let n = min(s1.len(), s2.len());
        let (mut s1, mut s2) = (s1.chars(), s2.chars());
        while i < n {
            let (c1, c2) = (s1.next().unwrap(), s2.next().unwrap());
            if c1 == c2 {
                result.push(c1);
            } else {
                break;
            }
            i += 1;
        }
        return result;
    }
    return strs
        .into_iter()
        .reduce(|s1, s2| common_prefix(&s1, &s2))
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let strs = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ];
        assert_eq!(longest_common_prefix(strs), "fl".to_string());

        let strs = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
        assert_eq!(longest_common_prefix(strs), "".to_string());
    }
}
