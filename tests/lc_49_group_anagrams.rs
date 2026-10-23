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

#[cfg(test)]
mod tests {
    use super::Solution;

    fn words(values: &[&str]) -> Vec<String> {
        values.iter().map(|word| (*word).to_string()).collect()
    }

    fn normalize(mut groups: Vec<Vec<String>>) -> Vec<Vec<String>> {
        for group in &mut groups {
            group.sort();
        }
        groups.sort();
        groups
    }

    #[test]
    fn groups_words_with_the_same_letters() {
        let actual = Solution::group_anagrams(words(&[
            "eat", "tea", "tan", "ate", "nat", "bat",
        ]));
        let expected = vec![
            words(&["eat", "tea", "ate"]),
            words(&["tan", "nat"]),
            words(&["bat"]),
        ];
        assert_eq!(normalize(actual), normalize(expected));
    }

    #[test]
    fn handles_empty_inputs_and_empty_strings() {
        assert!(Solution::group_anagrams(vec![]).is_empty());
        assert_eq!(
            Solution::group_anagrams(vec![String::new()]),
            vec![vec![String::new()]]
        );
    }
}
