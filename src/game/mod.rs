use colored::{ColoredString, Colorize};

#[derive(Debug, PartialEq)]
pub enum MoveError {
    GameEndedError,
    OutOfBoundsError,
    IndexTakenError,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Player {
    Player1,
    Player2,
}

#[derive(Debug, PartialEq)]
pub enum Winner {
    Player(Player),
    Tie,
}

pub struct Game {
    turn: Player,
    board: [Option<Player>; 9],
}

static COLUMN_COUNT: usize = 3;
static WINNING_COMBOS: [[usize; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];

impl Game {
    pub fn new() -> Game {
        Game {
            turn: Player::Player1,
            board: [None; 9],
        }
    }

    pub fn board_to_colored_strings(&self) -> Vec<ColoredString> {
        let mut board_strings = Vec::new();
        let winning_combo = self.get_winning_combo();
        for (i, cell) in self.board.iter().enumerate() {
            match cell {
                Some(Player::Player1) => {
                    if winning_combo.is_some() && winning_combo.unwrap().contains(&i) {
                        board_strings.push("X".red().on_white())
                    } else {
                        board_strings.push("X".red())
                    }
                }
                Some(Player::Player2) => {
                    if winning_combo.is_some() && winning_combo.unwrap().contains(&i) {
                        board_strings.push("O".blue().on_white())
                    } else {
                        board_strings.push("O".blue())
                    }
                }
                None => board_strings.push((i + 1).to_string().truecolor(211, 211, 211)),
            }
            if i % COLUMN_COUNT == 2 {
                board_strings.push("\n".clear());
            } else {
                board_strings.push("|".clear());
            }
        }
        board_strings
    }

    pub fn board_to_string(&self) -> String {
        let mut board_string = String::new();
        for (i, cell) in self.board.iter().enumerate() {
            match cell {
                Some(Player::Player1) => board_string.push_str("X"),
                Some(Player::Player2) => board_string.push_str("O"),
                None => board_string.push_str(" "),
            }
            if i % COLUMN_COUNT == 2 {
                board_string.push_str("\n");
            } else {
                board_string.push_str("|");
            }
        }
        board_string
    }

    pub fn get_available_spaces(&self) -> Vec<usize> {
        let mut open_spaces = Vec::new();
        for (i, cell) in self.board.iter().enumerate() {
            if cell.is_none() {
                open_spaces.push(i);
            }
        }
        open_spaces
    }

    pub fn get_turn(&self) -> Player {
        self.turn
    }

    pub fn has_ended(&self) -> bool {
        self.get_winner().is_some() || self.no_more_moves()
    }

    pub fn no_more_moves(&self) -> bool {
        self.board.iter().filter(|x| x.is_some()).count() == 9
    }

    pub fn get_winning_combo(&self) -> Option<[usize; 3]> {
        for combo in WINNING_COMBOS {
            let mut player1_count = 0;
            let mut player2_count = 0;
            for index in combo {
                match self.board[index] {
                    Some(Player::Player1) => player1_count += 1,
                    Some(Player::Player2) => player2_count += 1,
                    None => (),
                }
            }
            // check if the current combo has been matched (3 in a row)
            if player1_count == combo.len() {
                return Some(combo);
            }
            if player2_count == combo.len() {
                return Some(combo);
            }
        }
        None
    }

    pub fn get_winner(&self) -> Option<Winner> {
        // check if any of the players have any of the winning combos
        let winning_combo = self.get_winning_combo();
        match winning_combo {
            Some(combo) => {
                let mut player1_count = 0;
                let mut player2_count = 0;
                for index in combo {
                    match self.board[index] {
                        Some(Player::Player1) => player1_count += 1,
                        Some(Player::Player2) => player2_count += 1,
                        None => (),
                    }
                }
                // check if the current combo has been matched (3 in a row)
                if player1_count == combo.len() {
                    return Some(Winner::Player(Player::Player1));
                }
                if player2_count == combo.len() {
                    return Some(Winner::Player(Player::Player2));
                }
            }
            None => (),
        }

        if self.no_more_moves() {
            return Some(Winner::Tie);
        }

        None
    }

    pub fn take_turn(&mut self, index: usize) -> Result<(), MoveError> {
        if self.has_ended() {
            return Err(MoveError::GameEndedError);
        }
        if index > self.board.len() - 1 {
            return Err(MoveError::OutOfBoundsError);
        }
        if self.board[index].is_some() {
            return Err(MoveError::IndexTakenError);
        }
        self.board[index] = Some(self.turn);
        match self.turn {
            Player::Player1 => self.turn = Player::Player2,
            Player::Player2 => self.turn = Player::Player1,
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::game::{Game, MoveError, Player, Winner};

    #[test]
    fn test_game() {
        let mut game = Game::new();
        assert_eq!(game.has_ended(), false);
        assert_eq!(game.get_winner(), None);

        assert!(game.take_turn(0).is_ok());
        assert_eq!(game.take_turn(0).unwrap_err(), MoveError::IndexTakenError);
        assert_eq!(game.take_turn(10).unwrap_err(), MoveError::OutOfBoundsError);
        let turns = [1, 2, 3, 4, 5, 6];
        for turn in turns.iter() {
            assert!(game.take_turn(*turn).is_ok());
        }

        // player 1 has won at this time
        assert_eq!(game.take_turn(7).unwrap_err(), MoveError::GameEndedError);
        assert_eq!(game.get_winner().unwrap(), Winner::Player(Player::Player1));
    }

    #[test]
    fn test_tied_game() {
        let mut game = Game::new();
        let turns = [0, 3, 1, 2, 4, 7, 5, 8, 6];
        for turn in turns.iter() {
            assert!(game.take_turn(*turn).is_ok());
        }
        assert_eq!(game.get_winner().unwrap(), Winner::Tie);
    }
}
