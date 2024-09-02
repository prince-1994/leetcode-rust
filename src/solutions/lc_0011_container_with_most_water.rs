pub fn max_area(height: Vec<i32>) -> i32 {
    let n = height.len();
    let (mut left, mut right) = (0, n - 1);
    let mut result = 0;
    while left < right {
        result = result.max(height[left].min(height[right]) * (right - left) as i32);
        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(max_area(vec![1, 1]), 1);
        assert_eq!(max_area(vec![4, 3, 2, 1, 4]), 16);
    }
}
