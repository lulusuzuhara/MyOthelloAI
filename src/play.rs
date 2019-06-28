use crate::color::*;
use crate::command::*;

pub type Board = [[Color; 10]; 10];

pub fn init_board () -> Board {
  let mut board = [[0; 10]; 10];
  for i in 0..10 {
    board[i][0] = sentinel;
    board[i][9] = sentinel;
    board[0][i] = sentinel;
    board[9][i] = sentinel;
  }
  board[4][4] = white;
  board[5][5] = white;
  board[4][5] = black;
  board[5][4] = black;

  board
}

pub fn number_of_stones (board: &Board) -> i32 {
  let mut num = 0;
  for i in 1..9 {
    for j in 1..9 {
      if board[i][j] != none {
        num += 1;
      }
    }
  }
  num
}

fn g (board: &Board, color: Color, (di, dj): (i32, i32), (i, j): (i32, i32), r: &mut Vec<(i32, i32)>) {
  let ocolor = opposite_color(color);

  if board[i as usize][j as usize] == ocolor {
    r.push((i, j));
    g(board, color, (di, dj), (i+di, j+dj), r);
  } else if board[i as usize][j as usize] == color {
  } else {
    r.clear();
  }
}

fn f (board: &Board, color: Color, (di, dj): (i32, i32), (i, j): (i32, i32), r: &mut Vec<(i32, i32)>) {
  let ocolor = opposite_color(color);

  if board[i as usize][j as usize] == ocolor {
    r.push((i, j));
    g(board, color, (di, dj), (i+di, j+dj), r)
  } else {
    r.clear();
  }
}

pub fn flippable_indices_line (board: &Board, color: Color, (di, dj): (i32, i32), (i, j): (i32, i32)) -> Vec<(i32, i32)> {
  let mut tmp = Vec::new();
  f(board, color, (di, dj), (i, j), &mut tmp);
  (*tmp).to_vec()
}

pub fn flippable_indices (board: &Board, color: Color, (i, j): (i32, i32)) -> Vec<(i32, i32)> {
  let dirs = vec![(-1,-1), (0, -1), (1,-1), (-1,0), (1,0), (-1,1), (0,1), (1,1)];

  let mut bs = Vec::new();
  for (di, dj) in dirs {
    bs.append(&mut flippable_indices_line(board, color, (di, dj), (i+di, j+dj)));
  }
  bs
}

pub fn flippable_num (board: &Board, color: Color, (i, j): (i32, i32)) -> usize {
  flippable_indices(board, color, (i, j)).len()
}

pub fn is_valid_move (board: &Board, color: Color, (i, j): (i32, i32)) -> bool {
  board[i as usize][j as usize] == none && flippable_num(board, color, (i, j)) != 0
}

pub fn do_move (board: &mut Board, com: &Move, color: Color) {
  match com {
    Move::Mv (i, j) => {
      let ms = flippable_indices(board, color, (*i, *j));
      for (ii,jj) in &ms {
        board[*ii as usize][*jj as usize] = color;
      }
      board[*i as usize][*j as usize] = color;
    }
    _ => {}
  }
}

pub fn mix (xs: Vec<i32>, ys: Vec<i32>) -> Vec<(i32, i32)> {
  let mut ans_v = Vec::new();
  for i in &xs {
    for j in &ys {
      ans_v.push((*i, *j));
    }
  }
  ans_v
}

pub fn valid_moves (board: &Board, color: Color) -> Vec<((i32, i32), usize)> {
  let ls1 = vec![1, 2, 3, 4, 5, 6, 7, 8];
  let ls2 = vec![1, 2, 3, 4, 5, 6, 7, 8];

  let mut ans_v = Vec::new();
  for tup in &mix(ls1, ls2) {
    if is_valid_move(board, color, *tup) {
      ans_v.push((*tup, flippable_num(board, color, *tup)));
    }
  }
  ans_v
}

/**
 * もし自分がその手に動かした時に、相手が隅を取れるような置き方になってしまうかどうか
 */
pub fn is_next_corner_taken (board: &mut Board, color: Color, (i, j): (i32, i32)) -> bool {
  do_move(board, &Move::Mv(i, j), color);

  let ms = valid_moves(&board, opposite_color(color));
  for ((i, j), _) in &ms {
    if (*i, *j) == (1, 1) || (*i, *j) == (1, 8) || (*i, *j) == (8, 1) || (*i, *j) == (8, 8) {
      return true;
    }
  } 
  false
}


pub fn play (board: &Board, color: Color) -> Move {
  let mut ms = valid_moves(board, color);
  if ms == vec![] {
    Move::Pass
  } else {
    // 隅が取れる時にはとにかく取る
    for ((i, j), _) in &ms {
      if (*i, *j) == (1, 1) || (*i, *j) == (1, 8) || (*i, *j) == (8, 1) || (*i, *j) == (8, 8) {
        return Move::Mv(*i, *j);
      }
    } 

    // 序盤は石を取りすぎないよう、最も取る石の少ない手にする
    if number_of_stones(&board) <= 20 {
      ms.sort_by(|a, b| a.1.cmp(&b.1));
    } else {
      ms.sort_by(|a, b| b.1.cmp(&a.1));
    }

    for ((i, j), _) in &ms {
      let mut board_tmp = board.clone();
      // 次に相手に隅を取られるような置き方を回避する
      if !is_next_corner_taken(&mut board_tmp, color, (*i, *j)) {
        return Move::Mv(*i, *j)
      }
    }
    
    let (i, j) = ms[0].0;
    Move::Mv(i, j)
  }
}

//fn count (board: &Board, color: Color) -> i32 {
//  let mut s = 0;
//
//  for i in 1..9 {
//    for j in 1..9 {
//      if board[i][j] == color {
//        s += 1;
//      }
//    }
//  }
//  s
//}

pub fn print_board (board: &Board) {
  println!(" |A B C D E F G H ");
  println!("-+----------------");

  for j in 1..9 {
    print!("{}|", j);
    for i in 1..9 {
      print_color(board[i][j]);
      print!(" ");
    }
    print!("\n");
  }
  println!("  (X: Black,  O: White)");
}

//fn report_result (board: &Board) {
//  println!("========== Final Result ==========");
//  let bc = count(board, black);
//  let wc = count(board, white);
//
//  if bc > wc {
//    println!("*Black wins!*");
//  } else if bc < wc {
//    println!("*White wins!*");
//  } else {
//    println!("*Even*");
//  }
//
//  println!("Black: {}", bc);
//  println!("White: {}", wc);
//  print_board(board);
//}



#[test]
fn check() {
  let mut board = init_board();
  assert_eq!(flippable_num(&board, black, (3, 4)), 1);
  assert_eq!(flippable_num(&board, black, (3, 5)), 0);
  assert_eq!(flippable_num(&board, black, (2, 5)), 0);
  assert_eq!(flippable_num(&board, black, (6, 6)), 0);
  assert_eq!(flippable_num(&board, white, (6, 6)), 0);
  assert_eq!(flippable_num(&board, white, (4, 6)), 1);

  for i in 1..9 {
    for j in 1..9 {
      board[i][j] = white;
    }
  }
  board[1][1] = black;
  board[6][3] = black;
  board[8][1] = black;
  assert_eq!(flippable_indices (&board, black, (1, 8)), vec![(1, 7), (1, 6), (1, 5), (1, 4), (1, 3), (1, 2), (2, 7), (3, 6), (4, 5), (5, 4)]);
  assert_eq!(flippable_indices (&board, black, (6, 1)), vec![(5, 1), (4, 1), (3, 1), (2, 1), (7, 1), (6, 2)]);

  board = init_board();
  print_board(&board);
  assert_eq!(valid_moves(&board, black), vec![((3, 4), 1), ((4, 3), 1), ((5, 6), 1), ((6, 5), 1)]);
  for i in 4..9 {
    for j in 3..9 {
      board[i][j] = white;
    }
  }
  board[8][8] = black;
  board[5][4] = black;
  print_board(&board);
  let mut ms = valid_moves(&board, black);
  assert_eq!(ms, vec![((3, 2), 1), ((3, 3), 4), ((3, 4), 1), ((3, 6), 1), ((3, 8), 4), ((5, 2), 1), ((7, 2), 1), ((8, 2), 5)]);
  ms.sort_by(|a, b| b.1.cmp(&a.1));
  assert_eq!(ms[0].0, (8, 2));

  board = init_board();
  do_move(&mut board, &Move::Mv(4, 3), black);
  do_move(&mut board, &Move::Mv(3, 3), white);
  do_move(&mut board, &Move::Mv(2, 3), black);
  do_move(&mut board, &Move::Mv(2, 2), white);
  assert_eq!(number_of_stones(&board), 8);
  print_board(&board);
  let mut board_clone1 = board.clone();
  assert_eq!(is_next_corner_taken(&mut board_clone1, black, (2, 1)), true);
  print_board(&board);
  let mut board_clone2 = board.clone();
  assert_eq!(is_next_corner_taken(&mut board_clone2, black, (3, 4)), false);
  print_board(&board);
}