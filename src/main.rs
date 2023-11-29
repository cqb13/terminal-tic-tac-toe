use crossterm::{
    event::{read, Event, KeyCode, KeyEvent},
    terminal,
};
use std::env;
use std::time::Duration;

pub mod display;

use display::{
    display_welcome,
    game::{clear_board, display_board, display_selector_board},
    game_options,
};

pub enum Movement {
    Up,
    Down,
    Left,
    Right,
}

pub enum GameState {
    Running,
    Draw,
    Win,
}

pub struct GameConfig {
    pub game_mode: GameMode,
    pub difficulty: Difficulty,
}

impl GameConfig {
    pub fn new(game_mode: GameMode, difficulty: Difficulty) -> GameConfig {
        GameConfig {
            game_mode,
            difficulty,
        }
    }

    pub fn set_game_mode(&mut self, game_mode: GameMode) {
        self.game_mode = game_mode;
    }

    pub fn set_difficulty(&mut self, difficulty: Difficulty) {
        self.difficulty = difficulty;
    }
}

pub enum GameMode {
    SinglePlayer,
    MultiPlayer,
}

pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

pub struct Board;

impl Board {
    pub fn new() -> [[i8; 3]; 3] {
        [[0, 0, 0], [0, 0, 0], [0, 0, 0]]
    }

    pub fn place_marker(
        mut game_board: [[i8; 3]; 3],
        current_pos: [i8; 2],
        current_player: i8,
    ) -> [[i8; 3]; 3] {
        let y = current_pos[0];
        let x = current_pos[1];

        game_board[y as usize][x as usize] = current_player;
        game_board
    }

    pub fn get_row(game_board: [[i8; 3]; 3], row: i8) -> [i8; 3] {
        if row < 0 || row > 2 {
            panic!("Row {} does not exist!", row)
        }

        game_board[row as usize]
    }

    pub fn get_column(game_board: [[i8; 3]; 3], column: i8) -> [i8; 3] {
        let mut column_array = [0, 0, 0];

        if column < 0 || column > 2 {
            panic!("Column {} does not exist!", column)
        }

        for (i, row) in game_board.iter().enumerate() {
            column_array[i] = row[column as usize];
        }

        column_array
    }

    // 1 => diagonal from t.left to b.right
    // 2 => diagonal from t.right to b.left
    pub fn get_diagonal(game_board: [[i8; 3]; 3], diagonal: i8) -> [i8; 3] {
        let mut diagonal_array = [0, 0, 0];

        if diagonal == 1 {
            diagonal_array[0] = game_board[0][0];
            diagonal_array[2] = game_board[2][2];
        } else if diagonal == 2 {
            diagonal_array[0] = game_board[0][2];
            diagonal_array[2] = game_board[2][0];
        } else {
            panic!("Diagonal {} does not exist!", diagonal)
        }

        diagonal_array[1] = game_board[1][1];

        diagonal_array
    }
}

fn main() {
    //TODO: make loop of game loop for multi play
    display_welcome();
    game_options();

    game_loop();

    terminal::disable_raw_mode().expect("Failed to disable raw mode");
}

fn game_loop() {
    let mut game_board = Board::new();
    let mut current_player = 1;

    display_board(game_board);

    loop {
        game_board = player_turn(game_board, current_player);
        display_board(game_board);

        match check_win(game_board) {
            GameState::Running => {}
            GameState::Draw => {
                println!("The game ends in a draw!");
                break;
            }
            GameState::Win => {
                println!("Player {} has won the game!", { current_player });
                break;
            }
        }

        current_player = if current_player == 1 { 2 } else { 1 };
    }
}

fn player_turn(mut game_board: [[i8; 3]; 3], current_player: i8) -> [[i8; 3]; 3] {
    let mut current_pos = [1, 1];

    clear_board();
    display_selector_board(game_board, current_pos, current_player);

    loop {
        terminal::enable_raw_mode().expect("Failed to enable raw mode");
        current_pos = if let Ok(event) = read() {
            match event {
                Event::Key(KeyEvent { code, .. }) => match code {
                    KeyCode::Char('q') => {
                        terminal::disable_raw_mode().expect("Failed to disable raw mode");
                        println!("Quitting...");
                        std::process::exit(0);
                    }
                    KeyCode::Up => move_current_pos(current_pos, Movement::Up),
                    KeyCode::Down => move_current_pos(current_pos, Movement::Down),
                    KeyCode::Left => move_current_pos(current_pos, Movement::Left),
                    KeyCode::Right => move_current_pos(current_pos, Movement::Right),
                    KeyCode::Enter => {
                        terminal::disable_raw_mode().expect("Failed to disable raw mode");

                        if valid_move(game_board, current_pos) {
                            clear_board();
                            game_board =
                                Board::place_marker(game_board, current_pos, current_player);
                            break;
                        }
                        current_pos
                    }
                    _ => current_pos,
                },
                _ => current_pos,
            }
        } else {
            current_pos
        };

        terminal::disable_raw_mode().expect("Failed to disable raw mode");

        // Introduce a short delay only on Windows
        if env::consts::OS == "windows" {
            std::thread::sleep(Duration::from_millis(50));
        }

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
fn move_current_pos(mut current_pos: [i8; 2], movement_direction: Movement) -> [i8; 2] {
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

fn valid_move(game_board: [[i8; 3]; 3], current_pos: [i8; 2]) -> bool {
    let y = current_pos[0];
    let x = current_pos[1];

    if game_board[y as usize][x as usize] == 0 {
        return true;
    }

    false
}

fn check_win(game_board: [[i8; 3]; 3]) -> GameState {
    for i in 0..3 {
        if Board::get_row(game_board, i)[0] == Board::get_row(game_board, i)[1]
            && Board::get_row(game_board, i)[1] == Board::get_row(game_board, i)[2]
            && Board::get_row(game_board, i)[0] != 0
        {
            return GameState::Win;
        }
    }

    for i in 0..3 {
        if Board::get_column(game_board, i)[0] == Board::get_column(game_board, i)[1]
            && Board::get_column(game_board, i)[1] == Board::get_column(game_board, i)[2]
            && Board::get_column(game_board, i)[0] != 0
        {
            return GameState::Win;
        }
    }

    if Board::get_diagonal(game_board, 1)[0] == Board::get_diagonal(game_board, 1)[1]
        && Board::get_diagonal(game_board, 1)[1] == Board::get_diagonal(game_board, 1)[2]
        && Board::get_diagonal(game_board, 1)[0] != 0
    {
        return GameState::Win;
    }

    for row in game_board {
        for position in row {
            if position == 0 {
                return GameState::Running;
            }
        }
    }

    GameState::Draw
}
