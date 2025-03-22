use std::io;

const BOARD: [u8; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];

fn main() {
    println!("Welcome to Tic Tak Rust!\n");
    loop {
        print_board();
        println!("Player X: ");
        let choice = get_player_choice();
        place(choice);
        if is_winner() {
            println!("Player X Won!");
            break;
        }
        print_board();
        println!("Player O: ");
        let choice = get_player_choice();
        place(choice);
        if is_winner() {
            println!("Player O Won!");
            break;
        }
    }
    
}

fn is_winner() -> bool {
    false
}

fn print_board() {
    
}

fn place(choice: u8) -> bool {
   false 
}

fn get_player_choice() -> u8 {
    println!("Please enter a number (0-8):");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    let number: u8 = input.trim().parse().unwrap();
    number
}
