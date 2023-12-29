use crossterm::{cursor, terminal, ExecutableCommand};
use std::io;

use crate::{Board, BoardPiece, Player, Position};

pub fn display_board(game_board: &[[BoardPiece; 3]; 3]) {
    let mut rows = Vec::new();
    for row in game_board {
        rows.push(build_row_display(row));
    }

    println!("{}", rows[0]);
    println!("------------");
    println!("{}", rows[1]);
    println!("------------");
    println!("{}", rows[2]);
}

pub fn display_selector_board(
    game_board: &[[BoardPiece; 3]; 3],
    current_pos: Position,
    current_player: Player,
) {
    let y = current_pos.get_y();
    let x = current_pos.get_x();

    let marker: BoardPiece = if game_board[y as usize][x as usize] == BoardPiece::Empty {
        match current_player {
            Player::X => BoardPiece::XSelected,
            Player::O => BoardPiece::OSelected,
        }
    } else {
        BoardPiece::Taken
    };

    let new_game_board = Board::place_marker(game_board, current_pos, marker);
    display_board(&new_game_board);
}

// clears the last 5 lines (the amount of lines the board takes up)
pub fn clear_board() {
    for _ in 0..5 {
        io::stdout().execute(cursor::MoveUp(1)).unwrap();
        io::stdout()
            .execute(terminal::Clear(terminal::ClearType::CurrentLine))
            .unwrap();
    }
}

fn build_row_display(board_row: &[BoardPiece; 3]) -> String {
    let mut row_objects = Vec::new();

    for board_piece in board_row {
        row_objects.push(board_piece.get_board_piece());
    }

    let formatted_row = format!("{}|{}|{}", row_objects[0], row_objects[1], row_objects[2]);
    formatted_row
}
