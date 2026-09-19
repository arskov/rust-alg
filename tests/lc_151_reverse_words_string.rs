struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let splits: Vec<&str> = s.split_whitespace().rev().collect();
        splits.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn reverse_words_test() {
        let input = String::from("  one  two  three  ");
        let expected = String::from("three two one");
        assert_eq!(expected, Solution::reverse_words(input));
    }
}
