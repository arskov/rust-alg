pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let max = *candies.iter().max().unwrap();
    candies.iter().map(|&e| e + extra_candies >= max).collect()
}

#[cfg(test)]
mod tests {
    use crate::lc_1431_kids_with_candies::kids_with_candies;

    #[test]
    fn kids_with_candies_test() {
        let candies = vec![2, 3, 5, 1, 3];
        let extra_candies = 3;
        let expected = vec![true, true, true, false, true];
        assert_eq!(expected, kids_with_candies(candies, extra_candies));

        let candies = vec![4, 2, 1, 1, 2];
        let extra_candies = 1;
        let expected = vec![true, false, false, false, false];
        assert_eq!(expected, kids_with_candies(candies, extra_candies));

        let candies = vec![12, 1, 12];
        let extra_candies = 10;
        let expected = vec![true, false, true];
        assert_eq!(expected, kids_with_candies(candies, extra_candies));
    }
}
