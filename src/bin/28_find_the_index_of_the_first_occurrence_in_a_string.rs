fn str_str(haystack: String, needle: String) -> i32 {
    if let Some(index) = haystack.find(&needle) {
        return index as i32;
    }
    return -1;
}

fn main() {
    println!("{}", str_str("hello".to_string(), "lla".to_string()));
}
