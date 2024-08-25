fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '(' => stack.push(')'),
            '{' => stack.push('}'),
            '[' => stack.push(']'),
            _ if stack.is_empty() || c != stack.pop().unwrap() => return false,
            _ => (),
        }
    }
    return stack.is_empty();
}

fn main() {
    println!("{}", is_valid("()[]{}".to_string()));
    println!("{}", is_valid("(]".to_string()));
}
