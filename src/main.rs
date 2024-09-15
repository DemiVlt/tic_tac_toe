use rand::{random, thread_rng, Rng};
use std::io::stdin;
fn main() {
    let two_player = loop {
        let options = vec!["Play against computer", "Play against a player"];
        break match select("Select a mode.", options) {
            1 => false,
            2 => true,
            _ => continue,
        };
    };
    let max_depth = loop {
        if two_player {
            break 0;
        }
        let options = vec!["Easy", "Medium", "Hard"];
        break match select("Select a difficulty.", options) {
            1 => 0,
            2 => thread_rng().gen_range(6..=7),
            3 => i32::max_value(),
            _ => continue,
        };
    };
    let mut board = [[' '; 3]; 3];
    draw_board(&board);
    for turns in 0..9 {
        let player = match turns % 2 {
            0 => 'X',
            _ => 'O',
        };
        let (row, col) = match player {
            'X' => user_move(&board),
            _ => {
                if two_player {
                    user_move(&board)
                } else {
                    best_move(&mut board, max_depth)
                }
            }
        };
        board[row][col] = player;
        draw_board(&board);
        if check_win(&board, player) {
            println!("Player {player} Won!");
            return;
        }
    }
    println!("Cat's Game!");
}
fn select(category: &str, options: Vec<&str>) -> u8 {
    print!("{esc}c", esc = 27 as char);
    println!("{category}");
    for (index, option) in options.iter().enumerate() {
        print!("{}: {option}. ", index + 1);
    }
    println!();
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");
    return input.trim().parse().unwrap_or(0);
}
fn check_win(board: &[[char; 3]; 3], player: char) -> bool {
    board
        .iter()
        .any(|row| row.iter().all(|square| square == &player))
        || (0..3).any(|col| board.iter().all(|row| row[col] == player))
        || (0..3).all(|i| board[i][i] == player)
        || (0..3).all(|i| board[i][2 - i] == player)
}
fn draw_board(board: &[[char; 3]; 3]) {
    print!("{esc}c", esc = 27 as char);
    for (row, cols) in board.iter().enumerate() {
        for (col, square) in cols.iter().enumerate() {
            let color = match square {
                'X' => 91,
                _ => 94,
            };
            print!(" \x1b[{color}m{square}\x1b[0m ");
            if col != cols.len() - 1 {
                print!("|");
            }
        }
        println!();
        if row != board.len() - 1 {
            println!("{}", "-".repeat(11));
        }
    }
}
fn user_move(board: &[[char; 3]; 3]) -> (usize, usize) {
    loop {
        println!("Please enter a row and col from 1 to 3, ie. (row, col): ");
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");
        let coords = input
            .trim()
            .trim_matches(|c| "()".contains(c))
            .split(',')
            .map(|s| s.parse().unwrap_or(0))
            .collect::<Vec<usize>>();
        if coords.len() == 2
            && coords.iter().all(|i| &0 < i && i <= &board.len())
            && board[coords[0] - 1][coords[1] - 1] == ' '
        {
            return (coords[0] - 1, coords[1] - 1);
        }
    }
}
fn best_move(board: &mut [[char; 3]; 3], max_depth: i32) -> (usize, usize) {
    let mut best_score = -f32::INFINITY;
    let mut best_move = (board.len(), board.len());
    for row in 0..board.len() {
        for col in 0..board.len() {
            if board[row][col] != ' ' {
                continue;
            }
            board[row][col] = 'O';
            let score = minmax(board, false, 0, max_depth);
            board[row][col] = ' ';
            if score > best_score {
                best_score = score;
                best_move = (row, col);
            }
        }
    }
    best_move
}
fn minmax(board: &mut [[char; 3]; 3], is_maxing: bool, depth: i32, max_depth: i32) -> f32 {
    if depth >= max_depth {
        return -1.;
    }
    if check_win(board, 'O') {
        return 1.;
    }
    if check_win(board, 'X') {
        return -1.;
    }
    if board
        .iter()
        .all(|row| row.iter().all(|square| square != &' '))
    {
        return 0.;
    }
    let mut best_score = match is_maxing {
        true => -f32::INFINITY,
        false => f32::INFINITY,
    };
    for row in 0..board.len() {
        for col in 0..board.len() {
            if board[row][col] != ' ' {
                continue;
            }
            board[row][col] = match is_maxing {
                true => 'O',
                false => 'X',
            };
            let score = minmax(board, false, depth + 1, max_depth);
            board[row][col] = ' ';
            best_score = match is_maxing {
                true => score.max(best_score),
                false => score.min(best_score),
            }
        }
    }
    best_score
}