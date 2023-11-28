pub mod game;

pub fn display_welcome() {
    println!("==============================");
    println!("      Welcome to Tic-Tac-Toe  ");
    println!("      Created by cqb13        ");
    println!("      GitHub: github.com/cqb13");
    println!("==============================");
    print!("\n");

    println!("Controls:");
    println!("  Move with arrows (←↑↓→)");
    println!("  Enter to select");
    println!("  'q' to quit");
    print!("\n");

    println!("Instructions:");
    println!("  Use arrows to navigate the board.");
    println!("  Press Enter to place your marker.");
    println!("  Try to get three in a row horizontally, vertically, or diagonally.");
    println!("  First to three wins!");
    print!("\n");

    println!("Enjoy the game!");
    println!("==============================");
    print!("\n");
}
