use std::io;
use rand::prelude::IndexedRandom;

const BOARD_SIZE: usize = 9;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Player {
    X,
    O,
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match *self {
            Player::X => "X",
            Player::O => "O",
        })
    }
}

struct Game {
    board: [Option<Player>; BOARD_SIZE],
}

impl Game {
    fn new() -> Self {
        Game {
            board: [None; BOARD_SIZE],
        }
    }

    fn print_board(&self) {
        for row in 0..3 {
            for col in 0..3 {
                let index = row * 3 + col;
                let mark = self.board[index].map_or(index.to_string(), |p| p.to_string());
                print!("{} ", mark);
            }
            println!();
        }
    }

    fn place(&mut self, index: usize, player: Player) -> bool {
        if self.board[index].is_none() {
            self.board[index] = Some(player);
            true
        } else {
            false
        }
    }

    fn is_winner(&self, player: Player) -> bool {
        [
            [0, 1, 2], [3, 4, 5], [6, 7, 8], // rows
            [0, 3, 6], [1, 4, 7], [2, 5, 8], // columns
            [0, 4, 8], [2, 4, 6], // diagonals
        ]
            .iter()
            .any(|&line| line.iter().all(|&pos| self.board[pos] == Some(player)))
    }

    fn is_full(&self) -> bool {
        self.board.iter().all(|&cell| cell.is_some())
    }

    fn available_moves(&self) -> Vec<usize> {
        (0..BOARD_SIZE).filter(|&i| self.board[i].is_none()).collect()
    }
}

fn get_player_choice() -> Result<usize, String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().map_err(|_| "Invalid input! Try again.".to_string())
}

fn take_turn(game: &mut Game, player: Player, is_bot: bool) -> bool {
    if is_bot {
        println!("It's {}'s turn (Bot)!", player);
        let available_moves = game.available_moves();
        let random_move = available_moves.choose(&mut rand::rng()).unwrap(); // Random move
        println!("Bot chose position: {}", random_move);
        game.place(*random_move, player)
    } else {
        println!("It's {}'s turn!", player);
        match get_player_choice() {
            Ok(choice) if choice < BOARD_SIZE && game.place(choice, player) => true,
            _ => {
                println!("Invalid move! Try again.");
                false
            }
        }
    }
}

fn choose_game_mode() -> bool {
    println!("Do you want to play against the bot? (y/N)");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    match input.trim().to_lowercase().as_str() {
        "y" | "yes" => true, // One player mode (against the bot)
        "n" | "no" => false, // Two player mode
        _ => {
            false
        }
    }
}

fn main() {
    let is_bot_mode = choose_game_mode();
    let mut game = Game::new();
    let mut current_player = Player::X;
    let mut is_bot_turn = false;

    loop {
        game.print_board();

        // Bot only takes a turn if it's a bot game and it's O's turn.
        if take_turn(&mut game, current_player, is_bot_mode && is_bot_turn) {
            if game.is_winner(current_player) {
                game.print_board();
                println!("Player {} won!", current_player);
                break;
            }

            if game.is_full() {
                game.print_board();
                println!("It's a draw!");
                break;
            }

            // Switch players
            current_player = match current_player {
                Player::X => Player::O,
                Player::O => Player::X,
            };

            // Toggle bot's turn only if it's a bot game
            if is_bot_mode {
                is_bot_turn = !is_bot_turn; // Toggle bot's turn between X and O
            }
        }
    }
}
