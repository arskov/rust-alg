# Rust Algorithm Practice

Each LeetCode problem lives in a separate file in `tests/`. Cargo discovers these files automatically; no `main()` or module registration is needed. `src/lib.rs` is intentionally empty apart from a comment.

## Running solutions

```sh
cargo test
cargo test --test lc_37_sudoku_solver
cargo test --test lc_1268_word_suggestion_system shared_prefixes_are_sorted_and_limited_to_three
cargo test --test lc_37_sudoku_solver -- --nocapture
```

## Adding a problem

Create `tests/lc_<number>_<name>.rs`:

```rust
struct Solution;

// Paste the LeetCode submission here, including imports and helper types.
impl Solution {
    pub fn example(value: i32) -> i32 {
        value
    }
}

#[test]
fn example_case() {
    assert_eq!(Solution::example(42), 42);
}
```

Each file has its own namespace, so `Solution` and helper names can be reused. When submitting to LeetCode, copy the implementation and any required constants, imports, and helpers; leave out the local `struct Solution;`, tests, and types LeetCode already provides (such as `ListNode`).

## Problems

- [1: two sum](tests/lc_1_two_sum.rs)
- [23: merge k sorted lists](tests/lc_23_merge_k_sorted_lists.rs)
- [37: sudoku solver](tests/lc_37_sudoku_solver.rs)
- [49: group anagrams](tests/lc_49_group_anagrams.rs)
- [128: longest consecutive sequence](tests/lc_128_longest_consecutive_sequence.rs)
- [151: reverse words string](tests/lc_151_reverse_words_string.rs)
- [217: contains duplicate](tests/lc_217_contains_duplicate.rs)
- [242: valid anagram](tests/lc_242_valid_anagram.rs)
- [238: product array exept self](tests/lc_238_product_array_exept_self.rs)
- [334: increased triple sequence](tests/lc_334_increased_triple_sequence.rs)
- [345: reverse vowels string](tests/lc_345_reverse_vowels_string.rs)
- [347: top k frequent elements](tests/lc_347_top_k_frequent_elements.rs)
- [605: can place flowers](tests/lc_605_can_place_flowers.rs)
- [1071: common divisor strings](tests/lc_1071_common_divisor_strings.rs)
- [1268: word suggestion system](tests/lc_1268_word_suggestion_system.rs)
- [1431: kids with candies](tests/lc_1431_kids_with_candies.rs)
- [1768: merge strings alternately](tests/lc_1768_merge_strings_alternately.rs)
