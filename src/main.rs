use tictactoe::{run, game::Winner, game::Player};

fn main() {
    let winner = run();
    println!("");
    // TODO: refactor these nested match statements 
    match winner {
        Winner::Tie => {
            println!("It's a tie!");
        }
        Winner::Player(player) => {
            match player {
                Player::Player1 => {
                    println!("Player 1 wins!");
                }
                Player::Player2 => {
                    println!("Player 2 wins!");
                }
            }
        }
    }
 }
