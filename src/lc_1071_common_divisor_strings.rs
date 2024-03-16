pub fn gcd_of_strings(str1: String, str2: String) -> String {
    if str1.is_empty() || str2.is_empty() {
        return String::new();
    }
    let concat1 = format!("{}{}", str1, str2);
    let concat2 = format!("{}{}", str2, str1);
    if concat1 != concat2 {
        return String::new();
    }
    let l = gcd(str1.len(), str2.len());
    str1.chars().take(l).collect()
}

fn gcd(x: usize, y: usize) -> usize {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

#[cfg(test)]
mod tests {
    use super::gcd_of_strings;

    #[test]
    fn gcd_of_strings_test() {
        let a = String::from("ABCABC");
        let b = String::from("ABC");
        let expected = String::from("ABC");
        assert_eq!(expected, gcd_of_strings(a, b));

        let a = String::from("ABABAB");
        let b = String::from("ABAB");
        let expected = String::from("AB");
        assert_eq!(expected, gcd_of_strings(a, b));

        let a = String::from("EVER");
        let b = String::from("LAST");
        let expected = String::from("");
        assert_eq!(expected, gcd_of_strings(a, b));
    }
}
