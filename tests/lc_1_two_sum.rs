use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() <= 1 {
            return vec![];
        }
        let mut cache = HashMap::<i32, usize>::new();
        for (i, &v) in nums.iter().enumerate() {
            let lookup = target - v;
            if let Some(&j) = cache.get(&lookup) {
                return vec![j as i32, i as i32];
            }
            cache.insert(v, i);
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn leetcode_examples() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    }

    #[test]
    fn handles_negative_numbers_and_zero() {
        assert_eq!(Solution::two_sum(vec![-3, 4, 3, 90], 0), vec![0, 2]);
        assert_eq!(Solution::two_sum(vec![0, 4, 3, 0], 0), vec![0, 3]);
        assert_eq!(Solution::two_sum(vec![-1, -2, -3, -4, -5], -8), vec![2, 4]);
    }

    #[test]
    fn returns_empty_when_no_pair_exists() {
        assert!(Solution::two_sum(vec![], 0).is_empty());
        assert!(Solution::two_sum(vec![3], 6).is_empty());
        assert!(Solution::two_sum(vec![1, 2, 3], 7).is_empty());
    }

    #[test]
    fn does_not_reuse_the_same_element() {
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert!(Solution::two_sum(vec![3, 1], 6).is_empty());
    }
}
