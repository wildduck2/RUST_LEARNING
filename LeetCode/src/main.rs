use std::collections::HashSet;

fn main() {
  let board: Vec<Vec<char>> = vec![
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

  is_valid_sudoku(board);
}

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
  let mut v_map: HashSet<char> = HashSet::new();
  let mut h_map: HashSet<char> = HashSet::new();
  let mut g_map: HashSet<char> = HashSet::new();

  println!("sdfsdf");
  // TODO
  // 1-track column
  for (i, column) in board.iter().enumerate() {
    for (j, &row) in column.iter().enumerate() {
      let board_row_item: char = board[j][i];
      let grid_item: char =
        board[usize::from(j / 3) + usize::from(i / 3) * 3][(j % 3) + (i * 3) % 9];
      // checking row
      if row != '.' {
        if h_map.contains(&row) {
          return false;
        }
        h_map.insert(row);
      }
      // checking column
      if board_row_item != '.' {
        if v_map.contains(&board_row_item) {
          return false;
        }
        v_map.insert(board_row_item);
      }
      // checking for grids
      if grid_item != '.' {
        if g_map.contains(&grid_item) {
          return false;
        }
        g_map.insert(grid_item);
      }
    }
    // println!("{:?} \n", g_map);
    // println!("{:?} \n", v_map);
    // println!("{:?} \n", h_map);
    h_map.clear();
    v_map.clear();
    g_map.clear();
  }

  return true;
}

struct User {
  username: String,      // optional, like `username?: string` in TS
  email: Option<String>, // optional
  active: Option<bool>,
}
