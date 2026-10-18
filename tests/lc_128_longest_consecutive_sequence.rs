use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let lookup: HashSet<i32> = nums.into_iter().collect();
        let mut best = 0;

        for &n in &lookup {
            if lookup.contains(&(n - 1)) {
                continue;
            }

            let mut count = 1;
            let mut next = n + 1;
            while lookup.contains(&next) {
                count += 1;
                next += 1;
            }

            best = best.max(count);
        }

        best
    }
}
