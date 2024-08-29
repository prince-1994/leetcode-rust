pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let n = nums.len();
    let mut j = 0;
    for i in 0..n {
        if nums[i] != nums[j] {
            j += 1;
            nums[j] = nums[i];
        }
    }
    return (j + 1) as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn main() {
        let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 4, 4];
        assert_eq!(remove_duplicates(&mut nums), 5);
    }
}
