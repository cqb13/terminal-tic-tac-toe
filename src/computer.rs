use rand::Rng;

use crate::{valid_move, Board, BoardPiece, Column, Diagonal, Difficulty, Position, Row};

#[derive(Clone, Copy)]
pub struct ComputerMove {
    pub valid_move: bool,
    pub position: Position,
}

impl ComputerMove {
    pub fn new(valid_move: bool, position: Position) -> ComputerMove {
        ComputerMove {
            valid_move,
            position,
        }
    }

    pub fn set_valid_move(&mut self, valid_move: bool) {
        self.valid_move = valid_move;
    }

    pub fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    pub fn is_valid(&self) -> bool {
        self.valid_move
    }

    pub fn get_position(&self) -> Position {
        self.position
    }
}

pub fn computer_move(game_board: &[[BoardPiece; 3]; 3], difficulty: Difficulty) -> Position {
    match difficulty {
        Difficulty::Easy => return make_random_move(&game_board),
        Difficulty::Medium | Difficulty::Hard => {
            let computer_win = counter_player_win(&game_board, BoardPiece::O);
            if computer_win.is_valid() {
                return computer_win.get_position();
            }

            let player_win = counter_player_win(&game_board, BoardPiece::X);
            if player_win.is_valid() {
                return player_win.get_position();
            }

            if difficulty == Difficulty::Medium {
                return make_random_move(&game_board);
            } else {
                return find_best_move_hard_mode(&game_board).get_position();
            }
        }
    }
}

fn find_best_move_hard_mode(game_board: &[[BoardPiece; 3]; 3]) -> ComputerMove {
    // If the center is available, take it
    if valid_move(game_board, Position::new(1, 1)) {
        return ComputerMove::new(true, Position::new(1, 1));
    }

    // If the player has taken opposite corners, take an available side
    if player_has_opposite_corners(game_board) {
        let side_positions = vec![
            Position::new(0, 1),
            Position::new(1, 0),
            Position::new(1, 2),
            Position::new(2, 1),
        ];

        for pos in side_positions {
            if valid_move(game_board, pos) {
                return ComputerMove::new(true, pos);
            }
        }
    }

    // If the player has taken a corner, take the opposite corner
    let player_corners = vec![
        Position::new(0, 0),
        Position::new(0, 2),
        Position::new(2, 0),
        Position::new(2, 2),
    ];

    for corner in player_corners {
        if game_board[corner.get_y() as usize][corner.get_x() as usize] == BoardPiece::X {
            let opposite_corner = get_opposite_corner(corner);
            if valid_move(game_board, opposite_corner) {
                return ComputerMove::new(true, opposite_corner);
            }
        }
    }

    let position = make_random_move(game_board);
    ComputerMove::new(true, position)
}

fn player_has_opposite_corners(game_board: &[[BoardPiece; 3]; 3]) -> bool {
    let player_corners = vec![
        Position::new(0, 0),
        Position::new(0, 2),
        Position::new(2, 0),
        Position::new(2, 2),
    ];

    for i in 0..3 {
        if game_board[player_corners[i].get_y() as usize][player_corners[i].get_x() as usize]
            == BoardPiece::X
            && game_board[player_corners[(i + 2) % 4].get_y() as usize]
                [player_corners[(i + 2) % 4].get_x() as usize]
                == BoardPiece::X
        {
            return true;
        }
    }

    false
}

fn get_opposite_corner(corner: Position) -> Position {
    Position::new(2 - corner.get_y(), 2 - corner.get_x())
}

fn make_random_move(game_board: &[[BoardPiece; 3]; 3]) -> Position {
    loop {
        let y = rand::thread_rng().gen_range(0..3);
        let x = rand::thread_rng().gen_range(0..3);

        let current_pos = Position::new(y, x);

        if valid_move(&game_board, current_pos.clone()) {
            return current_pos;
        }
    }
}

// Checks if a player will win the game on the current turn. if they can, it returns the position to block them
// Also used to check if the computer can win the game on the current turn, by using block position as win position
fn counter_player_win(game_board: &[[BoardPiece; 3]; 3], win_player: BoardPiece) -> ComputerMove {
    match win_player {
        BoardPiece::X => {}
        BoardPiece::O => {}
        _ => panic!("Invalid player piece"),
    }

    // check rows
    for row in 0..3 {
        let row_type = match row {
            0 => Row::Top,
            1 => Row::Middle,
            2 => Row::Bottom,
            _ => panic!("Invalid row"),
        };

        let line = Board::get_row(game_board, row_type);

        let mut player_piece_count = 0;
        let mut empty_piece_count = 0;

        for piece in line {
            if piece == win_player {
                player_piece_count += 1;
            } else if piece == BoardPiece::Empty {
                empty_piece_count += 1;
            }
        }

        if player_piece_count == 2 && empty_piece_count == 1 {
            let empty_position = get_empty_position_from_line(line);
            if empty_position.is_some() {
                return ComputerMove::new(true, Position::new(row as i8, empty_position.unwrap()));
            }
        }
    }

    // check columns
    for column in 0..3 {
        let column_type = match column {
            0 => Column::Left,
            1 => Column::Middle,
            2 => Column::Right,
            _ => panic!("Invalid column"),
        };

        let line = Board::get_column(game_board, column_type);

        let mut player_piece_count = 0;
        let mut empty_piece_count = 0;

        for piece in line {
            if piece == win_player {
                player_piece_count += 1;
            } else if piece == BoardPiece::Empty {
                empty_piece_count += 1;
            }
        }

        if player_piece_count == 2 && empty_piece_count == 1 {
            let empty_position = get_empty_position_from_line(line);
            if empty_position.is_some() {
                return ComputerMove::new(
                    true,
                    Position::new(empty_position.unwrap(), column as i8),
                );
            }
        }
    }

    // check diagonals
    let line = Board::get_diagonal(game_board, Diagonal::TopLeftToBottomRight);

    let mut player_piece_count = 0;
    let mut empty_piece_count = 0;

    for piece in line {
        if piece == win_player {
            player_piece_count += 1;
        } else if piece == BoardPiece::Empty {
            empty_piece_count += 1;
        }
    }

    if player_piece_count == 2 && empty_piece_count == 1 {
        let empty_position = get_empty_position_from_line(line);
        if empty_position.is_some() {
            return ComputerMove::new(
                true,
                Position::new(empty_position.unwrap(), empty_position.unwrap()),
            );
        }
    }

    let line = Board::get_diagonal(game_board, Diagonal::TopRightToBottomLeft);

    let mut player_piece_count = 0;
    let mut empty_piece_count = 0;

    for piece in line {
        if piece == win_player {
            player_piece_count += 1;
        } else if piece == BoardPiece::Empty {
            empty_piece_count += 1;
        }
    }

    if player_piece_count == 2 && empty_piece_count == 1 {
        let empty_position = get_empty_position_from_line(line);
        if empty_position.is_some() {
            return ComputerMove::new(
                true,
                Position::new(empty_position.unwrap(), 2 - empty_position.unwrap()),
            );
        }
    }

    ComputerMove::new(false, Position::new(1, 1))
}

// assumes already checked that 1 empty piece in the row
fn get_empty_position_from_line(line: [BoardPiece; 3]) -> Option<i8> {
    for (index, piece) in line.iter().enumerate() {
        if piece == &BoardPiece::Empty {
            return Some(index as i8);
        }
    }

    None
}
