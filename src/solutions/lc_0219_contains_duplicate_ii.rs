use std::collections::HashMap;

pub fn contains_nearby_duplicate(nums: Vec<i32>, k: usize) -> bool {
    let n = nums.len();
    let mut map: HashMap<i32, usize> = HashMap::new();
    
    for i in 0..n {
        let num = nums[i];
        if let Some(index) = map.get(&num) {
            if (i - index) <= k {
                return true;
            }
        }
        map.insert(num, i);
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(contains_nearby_duplicate(vec![1, 2, 3, 1], 3), true);
        assert_eq!(contains_nearby_duplicate(vec![1, 0, 1, 1], 1), true);
        assert_eq!(contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2), false);
    }
}
