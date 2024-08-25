fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
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

fn main() {
    let mut nums = vec![3, 2, 2, 3, 4, 5];
    let val = 3;
    println!("{}", remove_element(&mut nums, val));
}
