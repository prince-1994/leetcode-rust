pub fn is_palindrome(num: i32) -> bool {
    if num < 0 {
        return false;
    }
    let mut result = 0;
    let mut x = num;
    while x > 0 {
        result *= 10;
        result += x % 10;
        x /= 10;
    }
    return result == num;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(is_palindrome(32), false);
        assert_eq!(is_palindrome(33), true);
    }
}
