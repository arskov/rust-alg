static VOWELS: [char; 10] = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

pub fn reverse_vowels(s: String) -> String {
    let mut tmp: Vec<u8> = s.clone().into_bytes();
    let mut l = 0;
    let mut r = s.len() - 1;
    while l < r {
        if !VOWELS.contains(&(tmp[l] as char)) {
            l += 1;
            continue;
        }
        if !VOWELS.contains(&(tmp[r] as char)) {
            r -= 1;
            continue;
        }
        tmp.swap(l, r);
        l += 1;
        r -= 1;
    }
    String::from_utf8_lossy(&tmp).to_string()
}

#[cfg(test)]
mod tests {
    use super::reverse_vowels;

    #[test]
    fn reverse_vowels_test() {
        let str1 = String::from("hello");
        assert_eq!(String::from("holle"), reverse_vowels(str1));

        let str1 = String::from("aeiou");
        assert_eq!(String::from("uoiea"), reverse_vowels(str1));

        let str1 = String::from("HaLE");
        assert_eq!(String::from("HELa"), reverse_vowels(str1));
    }
}
