pub fn merge_alternately(word1: String, word2: String) -> String {
    let w1_len = word1.len();
    let w2_len = word2.len();
    let mut word1 = word1.chars();
    let mut word2 = word2.chars();
    let mut out = String::with_capacity(w1_len + w2_len);
    loop {
        let c1 = word1.next();
        let c2 = word2.next();
        if c1.is_none() && c2.is_none() {
            break;
        }
        if c1.is_some() {
            out.push(c1.unwrap());
        }
        if c2.is_some() {
            out.push(c2.unwrap());
        }
    }
    out.to_string()
}

#[cfg(test)]
mod tests {

    use super::merge_alternately;

    #[test]
    fn merge_alternately_test() {
        let word1 = String::from("abc");
        let word2 = String::from("pqr");
        let expected = String::from("apbqcr");
        assert_eq!(expected, merge_alternately(word1, word2));

        let word1 = String::from("ab");
        let word2 = String::from("pqrs");
        let expected = String::from("apbqrs");
        assert_eq!(expected, merge_alternately(word1, word2));

        let word1 = String::from("abcd");
        let word2 = String::from("pq");
        let expected = String::from("apbqcd");
        assert_eq!(expected, merge_alternately(word1, word2));
    }
}
