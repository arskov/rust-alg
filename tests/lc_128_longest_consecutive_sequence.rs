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

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn examples() {
        assert_eq!(
            Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]),
            4
        );
        assert_eq!(
            Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]),
            9
        );
    }

    #[test]
    fn handles_duplicates_negative_values_and_empty_input() {
        assert_eq!(Solution::longest_consecutive(vec![-2, -1, 0, 1, 1]), 4);
        assert_eq!(Solution::longest_consecutive(vec![]), 0);
    }
}
