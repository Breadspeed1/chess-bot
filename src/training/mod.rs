use owlchess::{Board, Color, DrawReason, Outcome};
use owlchess::Outcome::{Draw, Win};
use crate::player::Agent;

struct Tournament {

}

struct Game {
    board: Board,
    white: Agent,
    black: Agent,
    moves: u32
}

impl Game {
    fn new(white: Agent, black: Agent) -> Game {
        Game {
            board: Board::initial(),
            white,
            black,
            moves: 0
        }
    }

    fn play_through(&mut self) -> (&Agent, u32) {
        while !self.advance() || self.moves > 75 {
            self.moves += 1;
        }

        match self.board.calc_outcome().unwrap_or(Draw(DrawReason::Moves75)).winner() {
            None => { todo!() },
            Some(winner) => {
                match winner {
                    White => (&self.white, self.moves),
                    Black => (&self.black, self.moves)
                }
            },
        }
    }

    fn advance(&mut self) -> bool {
        match self.side() {
            Color::White => {
                self.board = self.board.make_move(self.white.get_next_move(&self.board)).expect("failed to make move");
            }
            Color::Black => {
                self.board = self.board.make_move(self.black.get_next_move(&self.board)).expect("failed to make move");
            }
        }

        !self.board.has_legal_moves()
    }

    fn side(&self) -> Color {
        self.board.side()
    }
}