pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let sz = nums.len();
    let mut fwd = vec![1; sz + 1];
    let mut rev = vec![1; sz + 1];
    for i in 0..sz {
        fwd[i + 1] = fwd[i] * nums[i];
        rev[i + 1] = rev[i] * nums[sz - i - 1];
    }
    let mut res = vec![1; sz];
    for i in 0..sz {
        res[i] = fwd[i] * rev[sz - i - 1];
    }
    res
}

#[cfg(test)]
mod tests {
    use super::product_except_self;

    #[test]
    fn product_except_self_test() {
        assert_eq!(vec![24, 12, 8, 6], product_except_self(vec![1, 2, 3, 4]));
    }
}
