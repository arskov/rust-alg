use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut cache = HashSet::with_capacity(nums.len());
        for &v in &nums {
            if cache.contains(&v) {
                return true;
            }
            cache.insert(v);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn leetcode_examples() {
        assert!(Solution::contains_duplicate(vec![1, 2, 3, 1]));
        assert!(!Solution::contains_duplicate(vec![1, 2, 3, 4]));
        assert!(Solution::contains_duplicate(vec![
            1, 1, 1, 3, 3, 4, 3, 2, 4, 2
        ]));
    }

    #[test]
    fn empty_and_single_element_have_no_duplicates() {
        assert!(!Solution::contains_duplicate(vec![]));
        assert!(!Solution::contains_duplicate(vec![42]));
    }

    #[test]
    fn handles_negative_zero_and_extreme_values() {
        assert!(Solution::contains_duplicate(vec![-1, 0, -1]));
        assert!(Solution::contains_duplicate(vec![0, 0]));
        assert!(!Solution::contains_duplicate(vec![i32::MIN, 0, i32::MAX]));
        assert!(Solution::contains_duplicate(vec![
            i32::MAX,
            i32::MIN,
            i32::MAX
        ]));
    }
}
