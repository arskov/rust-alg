use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut out: HashMap<Vec<char>, Vec<String>> =
            HashMap::with_capacity(strs.len());

        for s in strs {
            let mut key: Vec<char> = s.chars().collect();
            key.sort_unstable();
            out.entry(key).or_default().push(s);
        }

        out.into_values().collect()
    }
}
