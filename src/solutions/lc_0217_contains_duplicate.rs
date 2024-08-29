use std::collections::HashSet;
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut set = HashSet::new();
    for num in nums {
        if set.contains(&num) {
            return true;
        }
        set.insert(num);
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(contains_duplicate(vec![1, 2, 3, 1]), true);
        assert_eq!(contains_duplicate(vec![1, 2, 3, 4]), false);
        assert_eq!(contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]), true);
    }
}
