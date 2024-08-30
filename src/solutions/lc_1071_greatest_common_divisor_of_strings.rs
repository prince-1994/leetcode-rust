pub fn gcd_of_strings(str1: String, str2: String) -> String {
    let res1 = str1.clone() + &str2;
    let res2 = str2.clone() + &str1;
    if res1 != res2 {
        return String::new();
    }
    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 {
            return a;
        } else {
            return gcd(b, a % b);
        }
    }

    let n = gcd(str1.len(), str2.len());
    return str1[..n].to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn test() {
        assert_eq!(
            gcd_of_strings("ABCABC".to_string(), "ABC".to_string()),
            "ABC".to_string()
        );
        assert_eq!(
            gcd_of_strings("ABABAB".to_string(), "ABABA".to_string()),
            "".to_string()
        );
        assert_eq!(
            gcd_of_strings("ABAB".to_string(), "ABABAB".to_string()),
            "AB".to_string()
        );
    }
}
