use crate::game::{Game, MoveError, Player, Winner};
use colored::Colorize;
use std::{error::Error, io};

pub mod game;

fn parse_input(input: &String) -> Result<usize, Box<dyn Error>> {
    let input = input.trim();
    let input = input.parse::<usize>()?;
    Ok(input - 1)
}

fn print_board(game: &Game) {
    for string in game.board_to_colored_strings() {
        print!("{}", string);
    }
}

pub fn play_game() {
    let mut game = Game::new();
    println!(
        "Welcome to Tic Tac Toe! (Player 1 is {}, Player 2 is {}).",
        "X".red(),
        "O".blue()
    );
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
                game.take_turn(input).unwrap_or_else(|err| match err {
                    MoveError::IndexTakenError => {
                        let available_spaces = game
                            .get_available_spaces()
                            .iter()
                            .map(|i| (i + 1).to_string())
                            .collect::<Vec<String>>();
                        println!("That space is already taken.");
                        println!(
                            "Please try again choosing any of these spaces: {}",
                            available_spaces.join(", ")
                        )
                    }
                    MoveError::OutOfBoundsError => {
                        println!("{} is not in the range 1-9. Please try again.", input + 1);
                    }
                    MoveError::GameEndedError => {
                        println!("The game has ended. Please start a new game.");
                    }
                });
            }
            Err(_) => {
                println!("{} is not a number. Please try again.", raw_input.trim());
            }
        }
    }

    // game has ended
    println!("");
    print_board(&game);

    // we know for sure the game has ended and we have a winner,
    // so we can unwrap the winner safely and return it
    let winner = game.get_winner().unwrap();
    println!("");
    // TODO: refactor these nested match statements

    match winner {
        Winner::Tie => {
            println!("It's a tie!");
        }
        Winner::Player(player) => match player {
            Player::Player1 => {
                println!("Player 1 wins!");
            }
            Player::Player2 => {
                println!("Player 2 wins!");
            }
        },
    }
}
