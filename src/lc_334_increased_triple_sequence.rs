pub fn increasing_triplet(nums: Vec<i32>) -> bool {
    let mut f = i32::MAX;
    let mut s = i32::MAX;
    for n in nums {
        if n <= f {
            f = n;
        } else if n <= s {
            s = n;
        } else {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::increasing_triplet;
    #[test]
    fn increasing_triplet_test() {
        assert_eq!(true, increasing_triplet(vec![1, 2, 3, 4, 5]));
        assert_eq!(false, increasing_triplet(vec![5, 4, 3, 2, 1]));
        assert_eq!(true, increasing_triplet(vec![2, 1, 5, 0, 4, 6]));
    }
}
