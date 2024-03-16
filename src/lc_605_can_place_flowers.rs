pub fn can_place_flowers(mut flowerbed: Vec<i32>, n: i32) -> bool {
    let sz = flowerbed.len();
    let mut n = n;
    if sz == 0 && n == 0 {
        return true;
    } else if sz == 0 {
        return false;
    }
    for i in 0..sz {
        if flowerbed[i] == 0
            && ((i == 0 && i == sz - 1)
                || (i > 0 && i < sz - 1 && flowerbed[i - 1] == 0 && flowerbed[i + 1] == 0)
                || (i == sz - 1 && flowerbed[i - 1] == 0)
                || (i == 0 && flowerbed[i + 1] == 0))
        {
            n -= 1;
            flowerbed[i] = 1;
        }
    }
    n <= 0
}

#[cfg(test)]
mod tests {
    use super::can_place_flowers;

    #[test]
    fn can_place_flowers_test() {
        let input = vec![1, 0, 0, 0, 1];
        assert_eq!(true, can_place_flowers(input, 1));
        let input = vec![1, 0, 0, 0, 1];
        assert_eq!(false, can_place_flowers(input, 2));
        let input = vec![1, 0, 0, 0, 1, 0, 0];
        assert_eq!(true, can_place_flowers(input, 2));
        let input = vec![0];
        assert_eq!(true, can_place_flowers(input, 1));
    }
}
