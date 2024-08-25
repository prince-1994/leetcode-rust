fn length_of_last_word(s: String) -> i32 {
    let val = match s.trim().split_whitespace().last() {
        Some(word) => word.len() as i32,
        None => 0,
    };
    return val;
}

fn main() {
    println!("{}", length_of_last_word(" Hello    World ".to_string()));
}
