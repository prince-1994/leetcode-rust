pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut j = 0;
    let n = nums.len();
    for i in 0..n {
        if nums[i] != val {
            nums[j] = nums[i];
            j += 1;
        }
    }
    return j as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn main() {
        let mut nums = vec![3, 2, 2, 3, 4, 5];
        assert_eq!(remove_element(&mut nums, 3), 4);

        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        assert_eq!(remove_element(&mut nums, 2), 5);
    }
}
