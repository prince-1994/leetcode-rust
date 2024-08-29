use std::collections::HashSet;

pub fn is_happy(n: i32) -> bool {
    let mut n = n;
    let mut seen: HashSet<i32> = HashSet::new();
    loop {
        let mut sum = 0;
        while n > 0 {
            sum += (n % 10).pow(2);
            n /= 10;
        }
        n = sum;
        if seen.contains(&n) {
            break;
        }
        seen.insert(n);
    }

    return n == 1;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(is_happy(19), true);
        assert_eq!(is_happy(2), false);
        assert_eq!(is_happy(7), true);
    }
}
