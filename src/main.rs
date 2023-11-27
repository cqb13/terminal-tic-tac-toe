

fn main() {
    game_loop();
}

fn game_loop() {
    let mut game_board = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];

    loop {
        display_board(game_board);
        player_turn(game_board);
        display_board(game_board);
        if check_win(game_board) {
            break;
        }
        computer_turn();
        display_board(game_board);
        if check_win(game_board) {
            break;
        }
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
            0 => row_objects.push("  "),
            1 => row_objects.push("🟢"),
            2 => row_objects.push("❌"),
            _ => panic!("Unknown position marker"),
        }
    }

    let formatted_row = format!("{}|{}|{}", row_objects[0], row_objects[1], row_objects[2]);
    formatted_row
}

fn player_turn(game_board: [[i32; 3]; 3]) {
    let mut current_pos = [1, 1];

    loop {
        listen_for_input(current_pos);
        
    }
}

fn listen_for_input(current_pos: [i32; 2]) {
    // done
}

fn computer_turn() {
    println!("Computer's turn");
}

fn check_win(game_board: [[i32; 3]; 3]) -> bool {
    // check for straight lines
    for row in game_board {
        if row == [1, 1, 1] || row == [2, 2, 2] {
            return true;
        }
    }

    // check for columns
    for i in 0..3 {
        if game_board[0][i] == game_board[1][i] && game_board[1][i] == game_board[2][i] {
            return true;
        }
    }

    // check for diagonals
    if game_board[0][0] == game_board[1][1] && game_board[1][1] == game_board[2][2] {
        return true;
    }

    if game_board[0][2] == game_board[1][1] && game_board[1][1] == game_board[2][0] {
        return true;
    }

    false
}

