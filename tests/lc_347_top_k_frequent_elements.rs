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
