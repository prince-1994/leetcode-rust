pub fn title_to_number(column_title: String) -> i32 {
    let mut result = 0;
    for c in column_title.chars() {
        result *= 26;
        result += ((c as u8 - 'A' as u8) + 1) as i32;
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(title_to_number(String::from("A")), 1);
        assert_eq!(title_to_number(String::from("AB")), 28);
        assert_eq!(title_to_number(String::from("ZY")), 701);
    }
}
