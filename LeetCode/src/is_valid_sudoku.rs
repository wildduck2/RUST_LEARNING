use std::collections::HashSet;

pub struct Solution;
impl Solution {
  pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut rows: Vec<HashSet<char>> = vec![HashSet::new(); 9];
    let mut columns: Vec<HashSet<char>> = vec![HashSet::new(); 9];
    let mut boxes: Vec<HashSet<char>> = vec![HashSet::new(); 9];

    for i in 0..9 {
      for j in 0..9 {
        let chunk = board[i][j];

        if chunk == '.' {
          continue;
        }

        if !rows[i].insert(chunk) {
          return false;
        }

        if !columns[j].insert(chunk) {
          return false;
        }

        /*
         * NOTE: The index of the 3x3 box is:
         * i / 3 * 3 + j / 3
         * where i is the row index and j is the column index
         * This is because we can divide the board into 3x3 boxes
         * 0 1 2
         * 3 4 5
         * 6 7 8
         * -------
         * 0 / 3 * 3 + 0 / 3 = 0
         * 1 / 3 * 3 + 1 / 3 = 1
         * ...etc
         */

        let bbox = (i / 3) * 3 + (j / 3);
        if !boxes[bbox].insert(chunk) {
          return false;
        }
      }
    }
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_valid_board() {
    let board = vec![
      vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
      vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
      vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
      vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
      vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
      vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
      vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
      vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
      vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    assert_eq!(Solution::is_valid_sudoku(board), true);
  }

  #[test]
  fn test_invalid_row() {
    let board = vec![
      vec!['5', '3', '.', '.', '7', '.', '.', '3', '.'], // duplicate '3' in row
      vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
      vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
      vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
      vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
      vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
      vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
      vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
      vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    assert_eq!(Solution::is_valid_sudoku(board), false);
  }

  #[test]
  fn test_invalid_column() {
    let board = vec![
      vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
      vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
      vec!['5', '9', '8', '.', '.', '.', '.', '6', '.'], // duplicate '5' in column 0
      vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
      vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
      vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
      vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
      vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
      vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    assert_eq!(Solution::is_valid_sudoku(board), false);
  }

  #[test]
  fn test_invalid_subgrid() {
    let board = vec![
      vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
      vec!['6', '.', '3', '1', '9', '5', '.', '.', '.'], // duplicate '3' in top-left 3x3 box
      vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
      vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
      vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
      vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
      vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
      vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
      vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    assert_eq!(Solution::is_valid_sudoku(board), false);
  }

  #[test]
  fn test_empty_board() {
    let board = vec![vec!['.'; 9]; 9];
    assert_eq!(Solution::is_valid_sudoku(board), true);
  }
}
