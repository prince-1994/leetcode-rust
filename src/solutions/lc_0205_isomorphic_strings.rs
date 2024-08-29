use std::collections::HashMap;
pub fn is_isomorphic(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let n = s.len();
    let mut s_map: HashMap<char, char> = HashMap::new();
    let mut t_map: HashMap<char, char> = HashMap::new();
    let mut s = s.chars();
    let mut t = t.chars();

    for _ in 0..n {
        let sc = s.next().unwrap();
        let tc = t.next().unwrap();
        if let Some(val) = s_map.get(&sc) {
            if val.to_owned() != tc {
                return false;
            }
        }
        if let Some(val) = t_map.get(&tc) {
            if val.to_owned() != sc {
                return false;
            }
        }
        s_map.insert(sc, tc);
        t_map.insert(tc, sc);
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(is_isomorphic("egg".to_string(), "add".to_string()), true);
        assert_eq!(is_isomorphic("foo".to_string(), "bar".to_string()), false);
        assert_eq!(
            is_isomorphic("paper".to_string(), "title".to_string()),
            true
        );
    }
}
