fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let n = nums.len();
    let (mut left, mut right) = (0 as i32, (n - 1) as i32);

    while left <= right {
        let mid = left + (right - left) / 2;
        if nums[mid as usize] == target {
            return mid as i32;
        } else if nums[mid as usize] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    return left as i32;
}

fn main() {
    println!("{}", search_insert(vec![1, 3, 5, 6], 5));
    println!("{}", search_insert(vec![1, 3, 5, 6], 0));
    println!("{}", search_insert(vec![1, 3, 5, 6], 7));
    println!("{}", search_insert(vec![1, 3, 5, 6], 6));
}
