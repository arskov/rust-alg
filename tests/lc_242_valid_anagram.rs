struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let sb = s.as_bytes();
        let tb = t.as_bytes();

        if sb.len() != tb.len() {
            return false;
        }

        let mut lookup = [0; 26];

        for i in 0..sb.len() {
            lookup[(sb[i] - b'a') as usize] += 1;
            lookup[(tb[i] - b'a') as usize] -= 1;
        }

        lookup.iter().all(|&count| count == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn recognizes_anagrams() {
        assert!(Solution::is_anagram("anagram".into(), "nagaram".into()));
        assert!(Solution::is_anagram("aabbcc".into(), "abcabc".into()));
        assert!(Solution::is_anagram(String::new(), String::new()));
    }

    #[test]
    fn rejects_different_characters_or_lengths() {
        assert!(!Solution::is_anagram("rat".into(), "car".into()));
        assert!(!Solution::is_anagram("ab".into(), "a".into()));
    }
}
