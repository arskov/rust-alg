struct Solution;

// LeetCode provides ListNode; keep this definition for local testing only.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

impl Solution {
    pub fn merge_k_lists(_lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        todo!("Implement merge k sorted lists")
    }
}

#[test]
#[ignore = "Starter template: implement merge_k_lists first"]
fn empty_lists() {
    assert_eq!(Solution::merge_k_lists(vec![]), None);
}
