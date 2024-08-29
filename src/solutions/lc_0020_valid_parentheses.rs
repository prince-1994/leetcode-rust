pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '(' => stack.push(')'),
            '{' => stack.push('}'),
            '[' => stack.push(']'),
            _ if stack.is_empty() || c != stack.pop().unwrap() => return false,
            _ => (),
        }
    }
    return stack.is_empty();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn test() {
        assert_eq!(is_valid("()[]{}".to_string()), true);
        assert_eq!(is_valid("(]".to_string()), false);
    }
}
