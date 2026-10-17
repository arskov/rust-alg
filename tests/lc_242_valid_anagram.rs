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
