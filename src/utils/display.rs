use crossterm::{cursor, terminal, ExecutableCommand};
use std::io;

pub fn display_board(game_board: [[i32; 3]; 3]) {
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
    mut game_board: [[i32; 3]; 3],
    current_pos: [i32; 2],
    current_player: i32,
) {
    let y = current_pos[0];
    let x = current_pos[1];

    let marker = if game_board[y as usize][x as usize] == 0 {
        current_player + 10
    } else {
        -1
    };

    game_board = place_marker(game_board, current_pos, marker);
    display_board(game_board);
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

fn build_row_display(board_row: [i32; 3]) -> String {
    let mut row_objects = Vec::new();

    for position in board_row {
        match position {
            -1 => row_objects.push(" ▫️ "),
            11 => row_objects.push(" 🅇 "),
            12 => row_objects.push(" ⓪ "),
            0 => row_objects.push("   "),
            1 => row_objects.push(" X "),
            2 => row_objects.push(" O "),
            _ => panic!("Unknown position marker"),
        }
    }

    let formatted_row = format!("{}|{}|{}", row_objects[0], row_objects[1], row_objects[2]);
    formatted_row
}

pub fn place_marker(
    mut game_board: [[i32; 3]; 3],
    current_pos: [i32; 2],
    current_player: i32,
) -> [[i32; 3]; 3] {
    let y = current_pos[0];
    let x = current_pos[1];

    game_board[y as usize][x as usize] = current_player;

    game_board
}
