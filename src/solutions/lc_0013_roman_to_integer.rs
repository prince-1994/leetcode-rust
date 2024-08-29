use std::collections::HashMap;

pub fn roman_to_int(s: &str) -> i32 {
    let char_to_int: HashMap<char, i32> = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);

    let mut result = 0;
    let mut prev_val = 0;
    for c in s.chars() {
        if let Some(val) = char_to_int.get(&c) {
            if *val > prev_val {
                result += *val - 2 * prev_val;
            } else {
                result += *val;
            }
            prev_val = *val;
        }
    }
    return result;
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(roman_to_int("III"), 3);
        assert_eq!(roman_to_int("IV"), 4);
        assert_eq!(roman_to_int("IX"), 9);
        assert_eq!(roman_to_int("LVIII"), 58);
        assert_eq!(roman_to_int("MCMXCIV"), 1994);
    }
}
