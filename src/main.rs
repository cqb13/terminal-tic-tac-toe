use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyEvent},
    terminal, ExecutableCommand,
};
use std::io;

pub enum Movement {
    Up,
    Down,
    Left,
    Right,
}

pub enum BoardType {
    Normal,
    Selector,
}

fn main() {
    game_loop();

    terminal::disable_raw_mode().expect("Failed to disable raw mode");
}

fn game_loop() {
    let mut game_board = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];
    let mut current_player = 1;

    display_board(game_board);

    loop {
        game_board = player_turn(game_board, current_player);
        display_board(game_board);
        if check_win(game_board) {
            break;
        }
        current_player = if current_player == 1 { 2 } else { 1 };
    }
}

fn display_board(game_board: [[i32; 3]; 3]) {
    let mut rows = Vec::new();
    for row in game_board {
        rows.push(build_row_display(row));
    }

    println!("{}", rows[0]);
    println!("--------");
    println!("{}", rows[1]);
    println!("--------");
    println!("{}", rows[2]);
}

fn build_row_display(board_row: [i32; 3]) -> String {
    let mut row_objects = Vec::new();

    for position in board_row {
        match position {
            10 => row_objects.push("🟦"),
            11 => row_objects.push("⏺️"),
            12 => row_objects.push("🆇"),
            0 => row_objects.push("  "),
            1 => row_objects.push("🟢"),
            2 => row_objects.push("❌"),
            _ => panic!("Unknown position marker"),
        }
    }

    let formatted_row = format!("{}|{}|{}", row_objects[0], row_objects[1], row_objects[2]);
    formatted_row
}

// clears the last 5 lines (the amount of lines the board takes up)
fn clear_board() {
    for _ in 0..5 {
        io::stdout().execute(cursor::MoveUp(1)).unwrap();
        io::stdout()
            .execute(terminal::Clear(terminal::ClearType::CurrentLine))
            .unwrap();
    }
}

fn player_turn(mut game_board: [[i32; 3]; 3], current_player: i32) -> [[i32; 3]; 3] {
    let mut current_pos = [1, 1];

    clear_board();
    display_selector_board(game_board, current_pos, current_player);

    loop {
        terminal::enable_raw_mode().expect("Failed to enable raw mode");
        current_pos = if let Ok(event) = read() {
            match event {
                Event::Key(KeyEvent {
                    code: KeyCode::Up,
                    modifiers: _,
                    kind: _,
                    state: _,
                }) => move_current_pos(current_pos, Movement::Up),
                Event::Key(KeyEvent {
                    code: KeyCode::Down,
                    modifiers: _,
                    kind: _,
                    state: _,
                }) => move_current_pos(current_pos, Movement::Down),
                Event::Key(KeyEvent {
                    code: KeyCode::Left,
                    modifiers: _,
                    kind: _,
                    state: _,
                }) => move_current_pos(current_pos, Movement::Left),
                Event::Key(KeyEvent {
                    code: KeyCode::Right,
                    modifiers: _,
                    kind: _,
                    state: _,
                }) => move_current_pos(current_pos, Movement::Right),
                Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    modifiers: _,
                    kind: _,
                    state: _,
                }) => {
                    terminal::disable_raw_mode().expect("Failed to disable raw mode");

                    if valid_move(game_board, current_pos) {
                        clear_board();
                        game_board = place_marker(game_board, current_pos, current_player);
                        break;
                    }
                    current_pos
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: _,
                    kind: _,
                    state: _,
                }) => {
                    terminal::disable_raw_mode().expect("Failed to disable raw mode");
                    panic!("User quit!");
                }
                _ => current_pos,
            }
        } else {
            current_pos
        };

        terminal::disable_raw_mode().expect("Failed to disable raw mode");

        clear_board();
        display_selector_board(game_board, current_pos, current_player);
    }

    terminal::disable_raw_mode().expect("Failed to disable raw mode");
    game_board
}

fn display_selector_board(mut game_board: [[i32; 3]; 3], current_pos: [i32; 2], current_player: i32) {
    // empty square selector 10
    // player 1 square selector 11
    // player 2 square selector 12
    game_board = place_marker(game_board, current_pos, current_player + 10);
    display_board(game_board);

}

/**
 *   X ------------------>
 * Y  
 * |  (0,0) | (0,1) | (0,2)
 * |  ---------------------
 * |  (1,0) | (1,1) | (1,2)
 * |  ---------------------
 * V  (2,0) | (2,1) | (2,2)
 */
fn move_current_pos(mut current_pos: [i32; 2], movement_direction: Movement) -> [i32; 2] {
    let y = current_pos[0];
    let x = current_pos[1];

    match movement_direction {
        Movement::Up => {
            if y - 1 >= 0 {
                current_pos[0] = y - 1
            } else {
                current_pos[0] = 2
            }
        }
        Movement::Down => {
            if y + 1 <= 2 {
                current_pos[0] = y + 1
            } else {
                current_pos[0] = 0
            }
        }
        Movement::Left => {
            if x - 1 >= 0 {
                current_pos[1] = x - 1
            } else {
                current_pos[1] = 2
            }
        }
        Movement::Right => {
            if x + 1 <= 2 {
                current_pos[1] = x + 1
            } else {
                current_pos[1] = 0
            }
        }
    }

    current_pos
}

fn valid_move(game_board: [[i32; 3]; 3], current_pos: [i32; 2]) -> bool {
    let y = current_pos[0];
    let x = current_pos[1];

    if game_board[y as usize][x as usize] == 0 {
        return true;
    }

    false
}

fn place_marker(
    mut game_board: [[i32; 3]; 3],
    current_pos: [i32; 2],
    current_player: i32,
) -> [[i32; 3]; 3] {
    let y = current_pos[0];
    let x = current_pos[1];

    game_board[y as usize][x as usize] = current_player;

    game_board
}

fn check_win(game_board: [[i32; 3]; 3]) -> bool {
    false
}
