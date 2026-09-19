use std::collections::BTreeMap;

#[derive(Default)]
struct Node {
    children: BTreeMap<char, Node>,
    text: Option<String>,
}

struct Solution;

impl Solution {
    fn populate(root: &mut Node, products: Vec<String>) {
        for product in products {
            let mut current = &mut *root;
            for c in product.chars() {
                current = current.children.entry(c).or_default();
            }
            current.text = Some(product);
        }
    }

    fn collect(node: &Node, suggestions: &mut Vec<String>) {
        if suggestions.len() == 3 {
            return;
        }
        if let Some(text) = &node.text {
            suggestions.push(text.clone());
        }
        // Ordered children produce lexicographically sorted suggestions.
        for child in node.children.values() {
            if suggestions.len() == 3 {
                break;
            }
            Self::collect(child, suggestions);
        }
    }

    pub fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        let mut root = Node::default();
        Self::populate(&mut root, products);
        let mut current = Some(&root);
        search_word
            .chars()
            .map(|c| {
                current = current.and_then(|node| node.children.get(&c));
                let mut suggestions = Vec::new();
                if let Some(node) = current {
                    Self::collect(node, &mut suggestions);
                }
                suggestions
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn products(values: &[&str]) -> Vec<String> {
        values.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn shared_prefixes_are_sorted_and_limited_to_three() {
        let input = products(&["mousepad", "monitor", "mouse", "mobile", "moneypot"]);
        let expected = vec![
            products(&["mobile", "moneypot", "monitor"]),
            products(&["mobile", "moneypot", "monitor"]),
            products(&["mouse", "mousepad"]),
            products(&["mouse", "mousepad"]),
            products(&["mouse", "mousepad"]),
        ];
        assert_eq!(
            Solution::suggested_products(input, "mouse".into()),
            expected
        );
    }

    #[test]
    fn exact_word_precedes_longer_words_and_missing_prefix_stays_empty() {
        let result =
            Solution::suggested_products(products(&["bags", "bag", "baggage"]), "bagsy".into());
        let first = products(&["bag", "baggage", "bags"]);
        assert_eq!(
            result,
            vec![
                first.clone(),
                first.clone(),
                first,
                products(&["bags"]),
                vec![]
            ]
        );
        assert_eq!(
            Solution::suggested_products(products(&["apple"]), "zap".into()),
            vec![Vec::<String>::new(); 3]
        );
    }

    #[test]
    fn empty_inputs() {
        assert!(Solution::suggested_products(products(&["mouse"]), String::new()).is_empty());
        assert_eq!(
            Solution::suggested_products(vec![], "abc".into()),
            vec![Vec::<String>::new(); 3]
        );
    }
}
