use crossterm::{
    event::{read, Event, KeyCode, KeyEvent},
    terminal,
};
use rand::Rng;

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
    display_welcome();
    let config = game_options();

    let game_mode = config.game_mode;
    let difficulty = config.difficulty;

    let single_player = match game_mode {
        GameMode::SinglePlayer => true,
        GameMode::MultiPlayer => false,
    };

    let difficulty_num = match difficulty {
        Difficulty::Easy => 1,
        Difficulty::Medium => 2,
        Difficulty::Hard => 3,
    };

    game_loop(single_player, difficulty_num);

    terminal::disable_raw_mode().expect("Failed to disable raw mode");
}

fn game_loop(single_player: bool, difficulty: i8) {
    let mut game_board = Board::new();
    let mut current_player = 1;

    display_board(game_board);

    if single_player {
        //TODO: on medium and hard, randomize who goes first
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

            game_board = computer_turn(game_board, difficulty);
            clear_board();
            display_board(game_board);

            match check_win(game_board) {
                GameState::Running => {}
                GameState::Draw => {
                    println!("The game ends in a draw!");
                    break;
                }
                GameState::Win => {
                    println!("The computer has won the game!");
                    break;
                }
            }
        }
    } else {
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

fn computer_turn(mut game_board: [[i8; 3]; 3], difficulty: i8) -> [[i8; 3]; 3] {

    let computer_move = computer_move(game_board, difficulty);

    game_board = Board::place_marker(game_board, computer_move, 2);

    game_board
}

fn computer_move(game_board: [[i8; 3]; 3], difficulty: i8) -> [i8; 2] {
    match difficulty {
        1 => return make_random_move(game_board),
        2 | 3 => {

            let can_o_win = can_player_win(game_board, 2);
            let can_x_win = can_player_win(game_board, 1);

            if difficulty == 2 && miss_the_right_move() {
                return make_random_move(game_board);
            }

            if can_o_win[0] != -1 {
                return can_o_win;
            }

            if difficulty == 2 && miss_the_right_move() {
                return make_random_move(game_board);
            }

            if can_x_win[0] != -1 {
                return can_x_win;
            }

            if difficulty == 2 {
                return make_random_move(game_board);
            } else {
                return find_best_move_for_player(game_board, 2);
            }
        }
        _ => panic!("Invalid difficulty selected"),
    }
}

fn miss_the_right_move() -> bool {
    let mut rng = rand::thread_rng();
    let random_num = rng.gen_range(0..10);

    if random_num < 6 {
        return true;
    }

    false
}

fn make_random_move(game_board: [[i8; 3]; 3]) -> [i8; 2] {
    let mut current_pos = [1, 1];

    loop {
        let y = rand::thread_rng().gen_range(0..3);
        let x = rand::thread_rng().gen_range(0..3);

        current_pos[0] = y;
        current_pos[1] = x;

        if valid_move(game_board, current_pos) {
            break;
        }
    }
    current_pos
}

//TODO: when not lazy, make this good
fn find_best_move_for_player(game_board: [[i8; 3]; 3], player: i8) -> [i8; 2] {
    let mut best_move = [1, 1];

    // check rows
    for i in 0..3 {
        if Board::get_row(game_board, i)[0] == player
            && Board::get_row(game_board, i)[1] == 0
            && Board::get_row(game_board, i)[2] == 0
        {
            best_move[0] = i;
            best_move[1] = 1;
            return best_move;
        } else if Board::get_row(game_board, i)[0] == 0
            && Board::get_row(game_board, i)[1] == player
            && Board::get_row(game_board, i)[2] == 0
        {
            best_move[0] = i;
            best_move[1] = 0;
            return best_move;
        } else if Board::get_row(game_board, i)[0] == 0
            && Board::get_row(game_board, i)[1] == 0
            && Board::get_row(game_board, i)[2] == player
        {
            best_move[0] = i;
            best_move[1] = 0;
            return best_move;
        }
    }

    // check columns
    for i in 0..3 {
        if Board::get_column(game_board, i)[0] == player
            && Board::get_column(game_board, i)[1] == 0
            && Board::get_column(game_board, i)[2] == 0
        {
            best_move[0] = 1;
            best_move[1] = i;
            return best_move;
        } else if Board::get_column(game_board, i)[0] == 0
            && Board::get_column(game_board, i)[1] == player
            && Board::get_column(game_board, i)[2] == 0
        {
            best_move[0] = 0;
            best_move[1] = i;
            return best_move;
        } else if Board::get_column(game_board, i)[0] == 0
            && Board::get_column(game_board, i)[1] == 0
            && Board::get_column(game_board, i)[2] == player
        {
            best_move[0] = 0;
            best_move[1] = i;
            return best_move;
        }
    }

    // check diagonals
    if Board::get_diagonal(game_board, 1)[0] == player
        && Board::get_diagonal(game_board, 1)[1] == 0
        && Board::get_diagonal(game_board, 1)[2] == 0
    {
        best_move[0] = 1;
        best_move[1] = 1;
        return best_move;
    } else if Board::get_diagonal(game_board, 1)[0] == 0
        && Board::get_diagonal(game_board, 1)[1] == player
        && Board::get_diagonal(game_board, 1)[2] == 0
    {
        best_move[0] = 0;
        best_move[1] = 0;
        return best_move;
    } else if Board::get_diagonal(game_board, 1)[0] == 0
        && Board::get_diagonal(game_board, 1)[1] == 0
        && Board::get_diagonal(game_board, 1)[2] == player
    {
        best_move[0] = 0;
        best_move[1] = 0;
        return best_move;
    }

    best_move
}

// if there are two x's in a row, return the position of the empty spot
fn can_player_win(game_board: [[i8; 3]; 3], win_player: i8) -> [i8; 2] {
    let mut block_position = [-1, -1];

    // check rows
    for i in 0..3 {
        if Board::get_row(game_board, i)[0] == win_player
            && Board::get_row(game_board, i)[1] == win_player
            && Board::get_row(game_board, i)[2] == 0
        {
            block_position[0] = i;
            block_position[1] = 2;
            return block_position;
        } else if Board::get_row(game_board, i)[0] == win_player
            && Board::get_row(game_board, i)[1] == 0
            && Board::get_row(game_board, i)[2] == win_player
        {
            block_position[0] = i;
            block_position[1] = 1;
            return block_position;
        } else if Board::get_row(game_board, i)[0] == 0
            && Board::get_row(game_board, i)[1] == win_player
            && Board::get_row(game_board, i)[2] == win_player
        {
            block_position[0] = i;
            block_position[1] = 0;
            return block_position;
        }
    }

    // check columns
    for i in 0..3 {
        if Board::get_column(game_board, i)[0] == win_player
            && Board::get_column(game_board, i)[1] == win_player
            && Board::get_column(game_board, i)[2] == 0
        {
            block_position[0] = 2;
            block_position[1] = i;
            return block_position;
        } else if Board::get_column(game_board, i)[0] == win_player
            && Board::get_column(game_board, i)[1] == 0
            && Board::get_column(game_board, i)[2] == win_player
        {
            block_position[0] = 1;
            block_position[1] = i;
            return block_position;
        } else if Board::get_column(game_board, i)[0] == 0
            && Board::get_column(game_board, i)[1] == win_player
            && Board::get_column(game_board, i)[2] == win_player
        {
            block_position[0] = 0;
            block_position[1] = i;
            return block_position;
        }
    }

    // check diagonals
    if Board::get_diagonal(game_board, 1)[0] == win_player
        && Board::get_diagonal(game_board, 1)[1] == win_player  
        && Board::get_diagonal(game_board, 1)[2] == 0
    {
        block_position[0] = 2;
        block_position[1] = 2;
        return block_position;
    } else if Board::get_diagonal(game_board, 1)[0] == win_player
        && Board::get_diagonal(game_board, 1)[1] == 0
        && Board::get_diagonal(game_board, 1)[2] == win_player
    {
        block_position[0] = 1;
        block_position[1] = 1;
        return block_position;
    } else if Board::get_diagonal(game_board, 1)[0] == 0
        && Board::get_diagonal(game_board, 1)[1] == win_player
        && Board::get_diagonal(game_board, 1)[2] == win_player
    {
        block_position[0] = 0;
        block_position[1] = 0;
        return block_position;
    }

    block_position
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

    if Board::get_diagonal(game_board, 2)[0] == Board::get_diagonal(game_board, 2)[1]
        && Board::get_diagonal(game_board, 2)[1] == Board::get_diagonal(game_board, 2)[2]
        && Board::get_diagonal(game_board, 2)[0] != 0
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
