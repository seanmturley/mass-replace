use std::collections::HashMap;

pub fn replace_characters(input: &str, map: &HashMap<char, char>) -> String {
    input.chars().map(|c| *map.get(&c).unwrap_or(&c)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_replace_when_no_match_in_map() {
        let mut map = std::collections::HashMap::new();
        map.insert('z', 'y');

        let result = replace_characters("apple", &map);
        assert_eq!(result, "apple");
    }

    #[test]
    fn replace_single_character() {
        let mut map = std::collections::HashMap::new();
        map.insert('a', 'b');

        let result = replace_characters("apple", &map);
        assert_eq!(result, "bpple");
    }

    #[test]
    fn replace_multiple_characters() {
        let mut map = std::collections::HashMap::new();
        map.insert('a', 'x');
        map.insert('e', 'y');

        let result = replace_characters("apple", &map);
        assert_eq!(result, "xpply");
    }

    #[test]
    fn replace_despite_conflicts() {
        let mut map = std::collections::HashMap::new();
        map.insert('a', 'b');
        map.insert('b', 'c');

        let result = replace_characters("ab", &map);
        assert_eq!(result, "bc"); // 'a'->'b' & 'b'->'c' performed simultaneously
    }
}
