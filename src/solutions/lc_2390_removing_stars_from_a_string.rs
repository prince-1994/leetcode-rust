pub fn remove_stars(s: String) -> String {
    let mut result = String::new();
    for c in s.chars() {
        if c == '*' {
            result.pop();
        } else {
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
        assert_eq!(remove_stars("leet**cod*e".to_string()), "lecoe".to_string());
        assert_eq!(remove_stars("erase*****".to_string()), "".to_string());
        assert_eq!(remove_stars("a".to_string()), "a".to_string());
    }
}
