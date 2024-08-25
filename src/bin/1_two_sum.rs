fn two_sum(nums: Vec<i32>, target: i32) -> Vec<usize> {
    let n = nums.len();
    for i in 0..n {
        for j in i + 1..n {
            if nums[i] + nums[j] == target {
                return vec![i, j];
            }
        }
    }
    return vec![];
}

fn main() {
    let arr = vec![3, 2, 4, 5, 6, 9];
    let result = two_sum(arr, 8);
    println!("{:?}", result);
}
