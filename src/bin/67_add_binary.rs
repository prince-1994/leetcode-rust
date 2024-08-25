use std::cmp::max;

fn add_binary(a: String, b: String) -> String {
    let mut result = String::new();
    let mut carry = 0;
    let n = max(a.len(), b.len());
    let (mut a, mut b) = (a.chars().rev(), b.chars().rev());
    for _ in 0..n {
        let c1 = a.next().unwrap_or('0').to_digit(2).unwrap();
        let c2 = b.next().unwrap_or('0').to_digit(2).unwrap();

        let sum = c1 + c2 + carry;
        let new_char = if sum % 2 == 0 { '0' } else { '1' };
        result.push(new_char);
        carry = sum / 2;
    }
    if carry > 0 {
        result.push('1');
    }
    return result.chars().rev().collect::<String>();
}

fn main() {
    println!("{}", add_binary("11".to_string(), "1".to_string()));
    println!("{}", add_binary("1010".to_string(), "1011".to_string()));
}
