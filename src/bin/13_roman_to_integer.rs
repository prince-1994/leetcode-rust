use std::collections::HashMap;

fn roman_to_int(s: &str) -> i32 {
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
fn main() {
    println!("{}", roman_to_int("III")); // Output: 3
    println!("{}", roman_to_int("IV")); // Output: 4
    println!("{}", roman_to_int("IX")); // Output: 9
    println!("{}", roman_to_int("LVIII")); // Output: 58
    println!("{}", roman_to_int("MCMXCIV")); // Output: 1994
}
