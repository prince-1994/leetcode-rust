fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut carry = 1;

    for i in (0..digits.len()).rev() {
        let num = digits[i] + carry;
        result.push(num % 10);
        carry = num / 10;
    }
    if carry > 0 {
        result.push(carry);
    }

    result.reverse();
    return result;
}
fn main() {
    println!("{:?}", plus_one(vec![1, 2, 3]));
    println!("{:?}", plus_one(vec![9, 9, 9]));
}
