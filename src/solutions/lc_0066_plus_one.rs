pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn test() {
        assert_eq!(plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
        assert_eq!(plus_one(vec![9, 9, 9]), vec![1, 0, 0, 0]);
        assert_eq!(plus_one(vec![9, 9, 9, 9]), vec![1, 0, 0, 0, 0]);
        assert_eq!(plus_one(vec![1]), vec![2]);
    }
}
