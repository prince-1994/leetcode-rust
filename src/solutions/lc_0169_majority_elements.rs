pub fn majority_element(nums: Vec<i32>) -> i32 {
    let (mut count, mut result) = (0, nums[0]);
    for num in nums {
        if num == result {
            count += 1;
        } else {
            if count > 0 {
                count -= 1;
            } else {
                result = num;
                count = 1;
            }
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(majority_element(vec![3, 2, 3]), 3);
        assert_eq!(majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }
}
