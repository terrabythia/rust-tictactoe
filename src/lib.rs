use std::{io, error::Error};
use crate::game::{Winner, Game, MoveError};

pub mod game;

fn parse_input(input: &String) -> Result<usize, Box<dyn Error>> {
    let input = input.trim();
    let input = input.parse::<usize>()?;
    Ok(input - 1)
}

fn print_board(game: &Game) {
    for char in game.board_to_colored_strings() {
        print!("{}", char);
    }
}

pub fn play_game() -> Winner {
    let mut game = Game::new();
    println!("Welcome to Tic Tac Toe! (Player 1 is X, Player 2 is O)");
    println!("Please enter a number from 1 to 9 to make a move.");

    while !game.has_ended() {
        println!("");
        print_board(&game);
        println!("");
        println!("{:?}'s turn.", game.get_turn());
        let mut raw_input = String::new();
        // ignore any error from stdin.read_line
        // because we will handle errors by checking the input string
        io::stdin().read_line(&mut raw_input).unwrap_or(0);
        let input = parse_input(&raw_input);
        match input {
            Ok(input) => {
                // make the move in the game
                // only handle when there is an error, otherwise
                // we can continue to the next turn
                game.take_turn(input).unwrap_or_else(|err| {
                    match err {
                        MoveError::IndexTakenError => {
                            let available_spaces = game.get_available_spaces().iter().map(|i| (i + 1).to_string()).collect::<Vec<String>>();
                            println!("That space is already taken.");
                            println!("Please try again choosing any of these spaces: {}", available_spaces.join(", "))
                        }
                        MoveError::OutOfBoundsError => {
                            println!("{} is not in the range 1-9. Please try again.", input + 1);
                        }
                        MoveError::GameEndedError => {
                            println!("The game has ended. Please start a new game.");
                        }
                    }
                });
            }
            Err(_) => {
                println!("{} is not a number between 1 and 9. Please try again.", raw_input.trim());
            }
        }
    }

    // game has ended
    println!("");
    print_board(&game);

    // we know for sure the game has ended and we have a winner,
    // so we can unwrap the winner safely and return it
    game.get_winner().unwrap()
}