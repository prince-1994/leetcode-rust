pub fn convert_to_title(column_number: i32) -> String {
    let mut result = Vec::new();
    let mut col = column_number;
    while col > 0 {
        col -= 1;
        result.push((col % 26 + 65) as u8 as char);
        col /= 26;
    }
    return result.iter().rev().collect::<String>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(convert_to_title(1), String::from("A"));
        assert_eq!(convert_to_title(28), String::from("AB"));
        assert_eq!(convert_to_title(701), String::from("ZY"));
    }
}
