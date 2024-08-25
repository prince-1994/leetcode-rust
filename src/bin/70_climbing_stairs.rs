fn climb_stairs(n: i32) -> i32 {
    let mut dp = vec![1; n as usize + 1];
    dp[1] = 1;

    for i in 2..=n as usize {
        dp[i] = dp[i - 1] + dp[i - 2];
    }

    return dp[n as usize];
}

fn main() {
    println!("{}", climb_stairs(3));
    println!("{}", climb_stairs(4));
    println!("{}", climb_stairs(10));
}
