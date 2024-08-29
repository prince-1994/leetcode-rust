use std::cmp::min;

pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
    let n = triangle.len();
    let mut dp1 = triangle[0].clone();
    for i in 1..n {
        let mut dp2 = Vec::new();
        let m = triangle[i].len();
        for j in 0..m {
            if j == 0 {
                dp2.push(dp1[0] + triangle[i][0])
            } else if j == m - 1 {
                dp2.push(dp1[j - 1] + triangle[i][j])
            } else {
                dp2.push(min(dp1[j - 1], dp1[j]) + triangle[i][j])
            }
        }
        dp1 = dp2;
    }
    return dp1.iter().min().unwrap().to_owned();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn test() {
        let triangle = vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];
        assert_eq!(minimum_total(triangle), 11);
        let triangle = vec![vec![2]];
        assert_eq!(minimum_total(triangle), 2);
    }
}
