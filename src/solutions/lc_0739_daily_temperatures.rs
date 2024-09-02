pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<usize> {
    let n = temperatures.len();
    let mut result = vec![0; n];
    let mut stack: Vec<usize> = vec![];
    for i in 0..n {
        while !stack.is_empty() && temperatures[stack[stack.len() - 1]] < temperatures[i] {
            let j = stack.pop().unwrap();
            result[j] = i - j;
        }
        stack.push(i);
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn test() {
        assert_eq!(
            daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
        assert_eq!(daily_temperatures(vec![30, 40, 50, 60]), vec![1, 1, 1, 0]);
        assert_eq!(daily_temperatures(vec![30, 60, 90]), vec![1, 1, 0]);
    }
}
