struct Solution;

const BOARD_SIZE: usize = 9;

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        Self::backtrack(board, 0, 0);
    }

    fn is_valid(board: &[Vec<char>], row: usize, col: usize, value: char) -> bool {
        for i in 0..BOARD_SIZE {
            if board[row][i] == value || board[i][col] == value {
                return false;
            }
        }
        let box_row = row / 3 * 3;
        let box_col = col / 3 * 3;
        for r in box_row..box_row + 3 {
            for c in box_col..box_col + 3 {
                if board[r][c] == value {
                    return false;
                }
            }
        }
        true
    }

    fn backtrack(board: &mut Vec<Vec<char>>, row: usize, col: usize) -> bool {
        if row == BOARD_SIZE {
            return true;
        }
        let (next_row, next_col) = if col + 1 == BOARD_SIZE {
            (row + 1, 0)
        } else {
            (row, col + 1)
        };
        if board[row][col] != '.' {
            return Self::backtrack(board, next_row, next_col);
        }
        for value in '1'..='9' {
            if Self::is_valid(board, row, col, value) {
                board[row][col] = value;
                if Self::backtrack(board, next_row, next_col) {
                    return true;
                }
                board[row][col] = '.';
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn board(rows: [&str; 9]) -> Vec<Vec<char>> {
        rows.into_iter().map(|row| row.chars().collect()).collect()
    }

    fn solved() -> Vec<Vec<char>> {
        board([
            "534678912",
            "672195348",
            "198342567",
            "859761423",
            "426853791",
            "713924856",
            "961537284",
            "287419635",
            "345286179",
        ])
    }

    #[test]
    fn solves_example_with_backtracking() {
        let mut input = board([
            "53..7....",
            "6..195...",
            ".98....6.",
            "8...6...3",
            "4..8.3..1",
            "7...2...6",
            ".6....28.",
            "...419..5",
            "....8..79",
        ]);
        Solution::solve_sudoku(&mut input);
        assert_eq!(input, solved());
    }

    #[test]
    fn solved_board_is_unchanged() {
        let mut input = solved();
        Solution::solve_sudoku(&mut input);
        assert_eq!(input, solved());
    }

    #[test]
    fn fills_last_cell() {
        let mut input = solved();
        input[8][8] = '.';
        Solution::solve_sudoku(&mut input);
        assert_eq!(input, solved());
    }
}
