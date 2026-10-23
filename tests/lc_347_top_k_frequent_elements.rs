use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if k <= 0 {
            return vec![];
        }
        let k = k as usize;

        let mut cache = HashMap::<i32, usize>::with_capacity(nums.len());
        for value in nums {
            *cache.entry(value).or_default() += 1;
        }

        let mut heap = BinaryHeap::with_capacity(k + 1);
        for (value, freq) in cache {
            heap.push(Reverse((freq, value)));
            if heap.len() > k {
                heap.pop();
            }
        }

        heap.into_iter()
            .map(|Reverse((_freq, value))| value)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn returns_the_k_most_frequent_values() {
        let mut result = Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2);
        result.sort_unstable();
        assert_eq!(result, vec![1, 2]);

        assert_eq!(Solution::top_k_frequent(vec![1], 1), vec![1]);
    }

    #[test]
    fn any_values_at_the_frequency_tie_are_valid() {
        let result = Solution::top_k_frequent(vec![4, 4, 1, 1, 2, 2], 2);
        assert_eq!(result.len(), 2);
        assert!(result.iter().all(|value| [1, 2, 4].contains(value)));
        assert_ne!(result[0], result[1]);
    }

    #[test]
    fn handles_empty_input_and_zero_k() {
        assert!(Solution::top_k_frequent(vec![], 1).is_empty());
        assert!(Solution::top_k_frequent(vec![1, 1], 0).is_empty());
    }
}
