fn my_sqrt(x: i32) -> i32 {
    let (mut l, mut r) = (0, x);
    while l <= r {
        let mid = l + (r - l) / 2;

        if mid * mid == x {
            return mid;
        } else if mid * mid > x {
            r = mid - 1;
        } else {
            l = mid + 1;
        }
    }
    return r;
}

fn main() {
    println!("{}", my_sqrt(5));
    println!("{}", my_sqrt(8));
    println!("{}", my_sqrt(9));
}
