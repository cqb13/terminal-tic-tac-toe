use crossterm::{
    event::{read, Event, KeyCode, KeyEvent},
    terminal,
};

pub mod utils;

use utils::display::{clear_board, display_board, display_selector_board, place_marker};

pub enum Movement {
    Up,
    Down,
    Left,
    Right,
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

fn check_win(game_board: [[i32; 3]; 3]) -> bool {
    false
}
