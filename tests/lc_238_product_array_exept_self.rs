struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let sz = nums.len();
        let mut res = Vec::with_capacity(sz);
        let mut prefix = 1;

        for i in 0..sz {
            res.push(prefix);
            prefix *= nums[i];
        }

        prefix = 1;
        for i in (0..sz).rev() {
            res[i] *= prefix;
            prefix *= nums[i];
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn product_except_self_test() {
        assert_eq!(
            vec![24, 12, 8, 6],
            Solution::product_except_self(vec![1, 2, 3, 4])
        );
    }
}
