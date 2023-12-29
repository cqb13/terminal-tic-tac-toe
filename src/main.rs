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

#[derive(Clone, Copy)]
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

impl PartialEq for BoardPiece {
    fn eq(&self, other: &Self) -> bool {
        match self {
            BoardPiece::X => match other {
                BoardPiece::X => true,
                _ => false,
            },
            BoardPiece::XSelected => match other {
                BoardPiece::XSelected => true,
                _ => false,
            },
            BoardPiece::O => match other {
                BoardPiece::O => true,
                _ => false,
            },
            BoardPiece::OSelected => match other {
                BoardPiece::OSelected => true,
                _ => false,
            },
            BoardPiece::Taken => match other {
                BoardPiece::Taken => true,
                _ => false,
            },
            BoardPiece::Empty => match other {
                BoardPiece::Empty => true,
                _ => false,
            },
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

pub struct Line {
    first: BoardPiece,
    second: BoardPiece,
    third: BoardPiece,
}

pub enum LinePosition {
    First,
    Second,
    Third,
}

impl Line {
    pub fn new(first: BoardPiece, second: BoardPiece, third: BoardPiece) -> Line {
        Line {
            first,
            second,
            third,
        }
    }

    pub fn get_first(&self) -> BoardPiece {
        self.first
    }

    pub fn get_second(&self) -> BoardPiece {
        self.second
    }

    pub fn get_third(&self) -> BoardPiece {
        self.third
    }

    pub fn set_first(&mut self, first: BoardPiece) {
        self.first = first;
    }

    pub fn set_second(&mut self, second: BoardPiece) {
        self.second = second;
    }

    pub fn set_third(&mut self, third: BoardPiece) {
        self.third = third;
    }

    pub fn get_column(&self) -> [BoardPiece; 3] {
        [self.first, self.second, self.third]
    }
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

#[derive(Clone, Copy)]
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

pub struct Board;

impl Board {
    pub fn new() -> [[BoardPiece; 3]; 3] {
        [
            [BoardPiece::Empty, BoardPiece::Empty, BoardPiece::Empty],
            [BoardPiece::Empty, BoardPiece::Empty, BoardPiece::Empty],
            [BoardPiece::Empty, BoardPiece::Empty, BoardPiece::Empty],
        ]
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

    pub fn convert_board_row_to_line(board_row: [BoardPiece; 3]) -> Line {
        Line::new(
            board_row[0].clone(),
            board_row[1].clone(),
            board_row[2].clone(),
        )
    }

    pub fn get_row(game_board: &[[BoardPiece; 3]; 3], row: Row) -> Line {
        match row {
            Row::Top => Board::convert_board_row_to_line(game_board[0].clone()),
            Row::Middle => Board::convert_board_row_to_line(game_board[1].clone()),
            Row::Bottom => Board::convert_board_row_to_line(game_board[2].clone()),
        }
    }

    pub fn get_column(game_board: &[[BoardPiece; 3]; 3], column: Column) -> Line {
        let mut column_array = Line::new(BoardPiece::Empty, BoardPiece::Empty, BoardPiece::Empty);

        let column_number = match column {
            Column::Left => 0,
            Column::Middle => 1,
            Column::Right => 2,
        };

        for (i, row) in game_board.iter().enumerate() {
            match i {
                0 => column_array.set_first(row[column_number].clone()),
                1 => column_array.set_second(row[column_number].clone()),
                2 => column_array.set_third(row[column_number].clone()),
                _ => panic!("Invalid row number"),
            }
        }

        column_array
    }

    pub fn get_diagonal(game_board: &[[BoardPiece; 3]; 3], diagonal: Diagonal) -> Line {
        let mut diagonal_array = Line::new(BoardPiece::Empty, BoardPiece::Empty, BoardPiece::Empty);

        match diagonal {
            Diagonal::TopLeftToBottomRight => {
                diagonal_array.set_first(game_board[0][0].clone());
                diagonal_array.set_third(game_board[2][2].clone());
            }
            Diagonal::TopRightToBottomLeft => {
                diagonal_array.set_first(game_board[0][2].clone());
                diagonal_array.set_third(game_board[2][0].clone());
            }
        }

        diagonal_array.set_second(game_board[1][1].clone());

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

fn computer_move(game_board: &[[BoardPiece; 3]; 3], difficulty: Difficulty) -> Position {
    match difficulty {
        Difficulty::Easy => return make_random_move(&game_board),
        Difficulty::Medium | Difficulty::Hard => {
            if difficulty == Difficulty::Medium && miss_the_right_move() {
                return make_random_move(&game_board);
            }

            let winning_move_for_o = counter_player_win(&game_board, Player::O.get_board_piece());

            if winning_move_for_o.is_valid()
                && valid_move(game_board, winning_move_for_o.get_position())
            {
                return winning_move_for_o.get_position();
            }

            if difficulty == Difficulty::Medium && miss_the_right_move() {
                return make_random_move(&game_board);
            }

            let block_x_win_move = counter_player_win(&game_board, Player::X.get_board_piece());

            if block_x_win_move.is_valid()
                && valid_move(game_board, block_x_win_move.get_position())
            {
                return block_x_win_move.get_position();
            }

            if difficulty == Difficulty::Medium {
                return make_random_move(&game_board);
            } else {
                let winning_move_for_o =
                    find_best_move_for_player(game_board, Player::O.get_board_piece());

                if winning_move_for_o.is_valid()
                    && valid_move(game_board, winning_move_for_o.get_position())
                {
                    return winning_move_for_o.get_position();
                } else {
                    return make_random_move(&game_board);
                }
            }
        }
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

fn make_random_move(game_board: &[[BoardPiece; 3]; 3]) -> Position {
    let mut current_pos = Position::new(1, 1);

    loop {
        let y = rand::thread_rng().gen_range(0..3);
        let x = rand::thread_rng().gen_range(0..3);

        current_pos.set_y(y);
        current_pos.set_x(x);

        if valid_move(&game_board, current_pos.clone()) {
            break;
        }
    }
    current_pos
}

fn find_best_move_for_player(
    game_board: &[[BoardPiece; 3]; 3],
    player: BoardPiece,
) -> ComputerMove {
    for i in 0..3 {
        let row = match i {
            0 => Row::Top,
            1 => Row::Middle,
            2 => Row::Bottom,
            _ => panic!("Invalid row number"),
        };

        if Board::get_row(game_board, row).get_first() == player
            && Board::get_row(game_board, row).get_second() == BoardPiece::Empty
            && Board::get_row(game_board, row).get_third() == BoardPiece::Empty
        {
            return ComputerMove::new(true, Position::new(i, 1));
        } else if Board::get_row(game_board, row).get_first() == BoardPiece::Empty
            && Board::get_row(game_board, row).get_second() == player
            && Board::get_row(game_board, row).get_third() == BoardPiece::Empty
        {
            return ComputerMove::new(true, Position::new(i, 0));
        } else if Board::get_row(game_board, row).get_first() == BoardPiece::Empty
            && Board::get_row(game_board, row).get_second() == BoardPiece::Empty
            && Board::get_row(game_board, row).get_third() == player
        {
            return ComputerMove::new(true, Position::new(i, 2));
        }
    }

    for i in 0..3 {
        let column = match i {
            0 => Column::Left,
            1 => Column::Middle,
            2 => Column::Right,
            _ => panic!("Invalid column number"),
        };

        if Board::get_column(game_board, column).get_first() == player
            && Board::get_column(game_board, column).get_second() == BoardPiece::Empty
            && Board::get_column(game_board, column).get_third() == BoardPiece::Empty
        {
            return ComputerMove::new(true, Position::new(1, i));
        } else if Board::get_column(game_board, column).get_first() == BoardPiece::Empty
            && Board::get_column(game_board, column).get_second() == player
            && Board::get_column(game_board, column).get_third() == BoardPiece::Empty
        {
            return ComputerMove::new(true, Position::new(0, i));
        } else if Board::get_column(game_board, column).get_first() == BoardPiece::Empty
            && Board::get_column(game_board, column).get_second() == BoardPiece::Empty
            && Board::get_column(game_board, column).get_third() == player
        {
            return ComputerMove::new(true, Position::new(2, i));
        }
    }

    if Board::get_diagonal(game_board, Diagonal::TopLeftToBottomRight).first == player
        && Board::get_diagonal(game_board, Diagonal::TopLeftToBottomRight).second
            == BoardPiece::Empty
        && Board::get_diagonal(game_board, Diagonal::TopLeftToBottomRight).third
            == BoardPiece::Empty
    {
        return ComputerMove::new(true, Position::new(1, 1));
    } else if Board::get_diagonal(game_board, Diagonal::TopLeftToBottomRight).first
        == BoardPiece::Empty
        && Board::get_diagonal(game_board, Diagonal::TopLeftToBottomRight).second == player
        && Board::get_diagonal(game_board, Diagonal::TopLeftToBottomRight).third
            == BoardPiece::Empty
    {
        return ComputerMove::new(true, Position::new(0, 0));
    } else if Board::get_diagonal(game_board, Diagonal::TopLeftToBottomRight).first
        == BoardPiece::Empty
        && Board::get_diagonal(game_board, Diagonal::TopLeftToBottomRight).second
            == BoardPiece::Empty
        && Board::get_diagonal(game_board, Diagonal::TopLeftToBottomRight).third == player
    {
        return ComputerMove::new(true, Position::new(2, 2));
    }

    ComputerMove::new(false, Position::new(1, 1))
}

// Checks if a player will win the game on the current turn. if they can, it returns the position to block them
// Also used to check if the computer can win the game on the current turn, by using block position as win position
fn counter_player_win(game_board: &[[BoardPiece; 3]; 3], win_player: BoardPiece) -> ComputerMove {
    match win_player {
        BoardPiece::X => {}
        BoardPiece::O => {}
        _ => panic!("Invalid player piece"),
    }

    for i in 0..3 {
        let row = match i {
            0 => Row::Top,
            1 => Row::Middle,
            2 => Row::Bottom,
            _ => panic!("Invalid row number"),
        };

        let row_line_details = check_line_for_win(Board::get_row(&game_board, row));
        if row_line_details.0 {
            let coordinate_x = match row_line_details.1 {
                LinePosition::First => 0,
                LinePosition::Second => 1,
                LinePosition::Third => 2,
            };

            return ComputerMove::new(true, Position::new(i, coordinate_x));
        }

        let column = match i {
            0 => Column::Left,
            1 => Column::Middle,
            2 => Column::Right,
            _ => panic!("Invalid column number"),
        };

        let column_line_details = check_line_for_win(Board::get_column(&game_board, column));
        if column_line_details.0 {
            let coordinate_y = match column_line_details.1 {
                LinePosition::First => 0,
                LinePosition::Second => 1,
                LinePosition::Third => 2,
            };

            return ComputerMove::new(true, Position::new(coordinate_y, i));
        }
    }

    let diagonal_line_details = check_line_for_win(Board::get_diagonal(
        &game_board,
        Diagonal::TopLeftToBottomRight,
    ));
    if diagonal_line_details.0 {
        let coordinate = match diagonal_line_details.1 {
            LinePosition::First => 0,
            LinePosition::Second => 1,
            LinePosition::Third => 2,
        };

        return ComputerMove::new(true, Position::new(coordinate, coordinate));
    }

    let diagonal_line_details = check_line_for_win(Board::get_diagonal(
        &game_board,
        Diagonal::TopRightToBottomLeft,
    ));
    if diagonal_line_details.0 {
        let coordinate = match diagonal_line_details.1 {
            LinePosition::First => 0,
            LinePosition::Second => 1,
            LinePosition::Third => 2,
        };

        return ComputerMove::new(true, Position::new(coordinate, coordinate));
    }

    ComputerMove::new(false, Position::new(1, 1))
}

fn check_line_for_win(line: Line) -> (bool, LinePosition) {
    let win_found = false;
    let win_position = LinePosition::First;

    if line.get_first() == line.get_second()
        && line.get_second() == line.get_third()
        && line.get_first() != BoardPiece::Empty
    {
        return (true, win_position);
    }

    if line.get_first() == line.get_second() && line.get_first() != BoardPiece::Empty {
        return (true, LinePosition::Third);
    }

    if line.get_first() == line.get_third() && line.get_first() != BoardPiece::Empty {
        return (true, LinePosition::Second);
    }

    if line.get_second() == line.get_third() && line.get_second() != BoardPiece::Empty {
        return (true, LinePosition::First);
    }

    (win_found, win_position)
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
        if line.get_first() == line.get_second()
            && line.get_second() == line.get_third()
            && line.get_first() != BoardPiece::Empty
        {
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
