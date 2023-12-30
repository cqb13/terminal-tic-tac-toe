use crossterm::{
    event::{read, Event, KeyCode, KeyEvent},
    terminal,
};

pub mod computer;
pub mod display;

use display::{
    display_welcome,
    game::{clear_board, display_board, display_selector_board},
    game_options,
};

use computer::computer_move;

pub enum Movement {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq)]
pub enum GameState {
    Running,
    Draw,
    Win,
}

#[derive(Clone, Copy)]
pub enum Player {
    X,
    O,
}

impl Player {
    pub fn get_player_piece(&self) -> String {
        match self {
            Player::X => "X".to_string(),
            Player::O => "O".to_string(),
        }
    }

    pub fn get_board_piece(&self) -> BoardPiece {
        match self {
            Player::X => BoardPiece::X,
            Player::O => BoardPiece::O,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BoardPiece {
    X,
    XSelected,
    O,
    OSelected,
    Taken,
    Empty,
}

impl BoardPiece {
    pub fn get_board_piece(&self) -> String {
        match self {
            BoardPiece::Taken => " ⌧ ".to_string(),
            BoardPiece::XSelected => " 🅇 ".to_string(),
            BoardPiece::OSelected => " ⓪ ".to_string(),
            BoardPiece::Empty => "   ".to_string(),
            BoardPiece::X => " X ".to_string(),
            BoardPiece::O => " O ".to_string(),
        }
    }
}

pub enum GameMode {
    SinglePlayer,
    MultiPlayer,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

pub enum Diagonal {
    TopLeftToBottomRight,
    TopRightToBottomLeft,
}

#[derive(Clone, Copy)]
pub enum Column {
    Left,
    Middle,
    Right,
}

#[derive(Clone, Copy)]
pub enum Row {
    Top,
    Middle,
    Bottom,
}

#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub y: i8,
    pub x: i8,
}

impl Position {
    pub fn clone(&self) -> Position {
        Position {
            y: self.y,
            x: self.x,
        }
    }

    pub fn new(y: i8, x: i8) -> Position {
        Position { y, x }
    }

    pub fn set_y(&mut self, y: i8) {
        self.y = y;
    }

    pub fn set_x(&mut self, x: i8) {
        self.x = x;
    }

    pub fn get_y(&self) -> i8 {
        self.y
    }

    pub fn get_x(&self) -> i8 {
        self.x
    }
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

pub enum BoardPosition {
    TopLeft,
    TopMiddle,
    TopRight,
    MiddleLeft,
    MiddleMiddle,
    MiddleRight,
    BottomLeft,
    BottomMiddle,
    BottomRight,
}

pub struct Board;

impl Board {
    pub fn new() -> [[BoardPiece; 3]; 3] {
        [
            [BoardPiece::Empty, BoardPiece::Empty, BoardPiece::Empty],
            [BoardPiece::Empty, BoardPiece::Empty, BoardPiece::Empty],
            [BoardPiece::Empty, BoardPiece::Empty, BoardPiece::Empty],
        ]
    }

    pub fn get_turn(game_board: &[[BoardPiece; 3]; 3]) -> usize {
        game_board
            .iter()
            .flatten()
            .filter(|&x| *x != BoardPiece::Empty)
            .count()
    }

    pub fn place_marker(
        game_board: &[[BoardPiece; 3]; 3],
        current_pos: Position,
        current_player_board_piece: BoardPiece,
    ) -> [[BoardPiece; 3]; 3] {
        let y = current_pos.get_y();
        let x = current_pos.get_x();

        let mut new_game_board = *game_board;
        new_game_board[y as usize][x as usize] = current_player_board_piece;
        new_game_board
    }

    pub fn get_board_piece_at_position(
        game_board: &[[BoardPiece; 3]; 3],
        position: BoardPosition,
    ) -> BoardPiece {
        match position {
            BoardPosition::TopLeft => game_board[0][0],
            BoardPosition::TopMiddle => game_board[0][1],
            BoardPosition::TopRight => game_board[0][2],
            BoardPosition::MiddleLeft => game_board[1][0],
            BoardPosition::MiddleMiddle => game_board[1][1],
            BoardPosition::MiddleRight => game_board[1][2],
            BoardPosition::BottomLeft => game_board[2][0],
            BoardPosition::BottomMiddle => game_board[2][1],
            BoardPosition::BottomRight => game_board[2][2],
        }
    }

    pub fn get_row(game_board: &[[BoardPiece; 3]; 3], row: Row) -> [BoardPiece; 3] {
        match row {
            Row::Top => game_board[0],
            Row::Middle => game_board[1],
            Row::Bottom => game_board[2],
        }
    }

    pub fn get_column(game_board: &[[BoardPiece; 3]; 3], column: Column) -> [BoardPiece; 3] {
        let mut column_array = [BoardPiece::Empty, BoardPiece::Empty, BoardPiece::Empty];

        let column_number = match column {
            Column::Left => 0,
            Column::Middle => 1,
            Column::Right => 2,
        };

        for i in 0..3 {
            column_array[i] = game_board[i][column_number].clone();
        }

        column_array
    }

    pub fn get_diagonal(game_board: &[[BoardPiece; 3]; 3], diagonal: Diagonal) -> [BoardPiece; 3] {
        let mut diagonal_array = [BoardPiece::Empty, BoardPiece::Empty, BoardPiece::Empty];

        match diagonal {
            Diagonal::TopLeftToBottomRight => {
                diagonal_array[0] = game_board[0][0];
                diagonal_array[2] = game_board[2][2];
            }
            Diagonal::TopRightToBottomLeft => {
                diagonal_array[0] = game_board[0][2];
                diagonal_array[2] = game_board[2][0];
            }
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

    game_loop(single_player, difficulty);

    terminal::disable_raw_mode().expect("Failed to disable raw mode");
}

fn game_loop(single_player: bool, difficulty: Difficulty) {
    let mut game_board = Board::new();
    let mut current_player = Player::X;

    display_board(&game_board);

    if single_player {
        //TODO: on medium and hard, randomize who goes first
        loop {
            game_board = player_turn(&game_board, current_player.clone());
            display_board(&game_board);

            match check_win(game_board) {
                GameState::Running => {}
                GameState::Draw => {
                    println!("The game ends in a draw!");
                    break;
                }
                GameState::Win => {
                    println!("Player {} has won the game!", {
                        current_player.get_player_piece()
                    });
                    break;
                }
            }

            game_board = computer_turn(game_board, difficulty.clone());
            clear_board();
            display_board(&game_board);

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
            game_board = player_turn(&game_board, current_player);
            display_board(&game_board);

            match check_win(game_board) {
                GameState::Running => {}
                GameState::Draw => {
                    println!("The game ends in a draw!");
                    break;
                }
                GameState::Win => {
                    println!("Player {} has won the game!", {
                        current_player.get_player_piece()
                    });
                    break;
                }
            }

            current_player = match current_player {
                Player::X => Player::O,
                Player::O => Player::X,
            }
        }
    }
}

fn player_turn(game_board: &[[BoardPiece; 3]; 3], current_player: Player) -> [[BoardPiece; 3]; 3] {
    let mut current_pos = Position::new(1, 1);

    clear_board();
    display_selector_board(&game_board, current_pos, current_player);

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

                        if valid_move(&game_board, current_pos) {
                            clear_board();
                            let updated_board = Board::place_marker(
                                &game_board,
                                current_pos,
                                current_player.get_board_piece(),
                            );
                            terminal::disable_raw_mode().expect("Failed to disable raw mode");
                            return updated_board;
                        }
                        current_pos.clone()
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
        display_selector_board(&game_board, current_pos, current_player);
    }
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
fn move_current_pos(mut current_pos: Position, movement_direction: Movement) -> Position {
    let y = current_pos.get_y();
    let x = current_pos.get_x();

    match movement_direction {
        Movement::Up => {
            if y - 1 >= 0 {
                current_pos.set_y(y - 1)
            } else {
                current_pos.set_y(2)
            }
        }
        Movement::Down => {
            if y + 1 <= 2 {
                current_pos.set_y(y + 1)
            } else {
                current_pos.set_y(0)
            }
        }
        Movement::Left => {
            if x - 1 >= 0 {
                current_pos.set_x(x - 1)
            } else {
                current_pos.set_x(2)
            }
        }
        Movement::Right => {
            if x + 1 <= 2 {
                current_pos.set_x(x + 1)
            } else {
                current_pos.set_x(0)
            }
        }
    }

    current_pos
}

fn computer_turn(
    mut game_board: [[BoardPiece; 3]; 3],
    difficulty: Difficulty,
) -> [[BoardPiece; 3]; 3] {
    let computer_move = computer_move(&game_board, difficulty);

    let updated_board = Board::place_marker(&game_board, computer_move, BoardPiece::O);
    game_board = updated_board;

    game_board
}

fn valid_move(game_board: &[[BoardPiece; 3]; 3], current_pos: Position) -> bool {
    let y = current_pos.get_y();
    let x = current_pos.get_x();

    if game_board[y as usize][x as usize] == BoardPiece::Empty {
        return true;
    }

    false
}

fn check_win(game_board: [[BoardPiece; 3]; 3]) -> GameState {
    let mut lines = Vec::new();

    for i in 0..3 {
        let row = match i {
            0 => Row::Top,
            1 => Row::Middle,
            2 => Row::Bottom,
            _ => panic!("Invalid row number"),
        };

        let column = match i {
            0 => Column::Left,
            1 => Column::Middle,
            2 => Column::Right,
            _ => panic!("Invalid column number"),
        };

        lines.push(Board::get_column(&game_board, column));
        lines.push(Board::get_row(&game_board, row));
    }

    lines.push(Board::get_diagonal(
        &game_board,
        Diagonal::TopLeftToBottomRight,
    ));

    lines.push(Board::get_diagonal(
        &game_board,
        Diagonal::TopRightToBottomLeft,
    ));

    for line in lines {
        if line[0] == line[1] && line[1] == line[2] && line[0] != BoardPiece::Empty {
            return GameState::Win;
        }
    }

    for row in game_board {
        for board_piece in row {
            if board_piece == BoardPiece::Empty {
                return GameState::Running;
            }
        }
    }

    GameState::Draw
}
