mod player;
mod training;

use std::thread::sleep;
use std::time::Duration;
use clearscreen::clear;
use owlchess::{Board, Color, DrawReason, Make, Outcome};
use owlchess::board::PrettyStyle;
use crate::player::Agent;
use crate::training::{Tournament, Trainer};

fn main() {
    let mut trainer: Trainer = Trainer::new(4096, 1024, 256, 0.001);

    for _ in 0..50 {
        trainer.run();
    }

    let mut one = trainer.get_from_recent(0);
    let mut two = trainer.get_from_recent(1);

    let mut board: Board = Board::initial();

    while board.has_legal_moves() {
        match board.side() {
            Color::White => { board = board.make_move(one.get_next_move(&board)).expect("failed to make move"); }
            Color::Black => { board = board.make_move(two.get_next_move(&board)).expect("failed to make move"); }
        }

        clear().expect("error clearing screen");
        println!("{}", board.pretty(PrettyStyle::Ascii));
        sleep(Duration::from_millis(500));
    }

    println!("{}", board.calc_outcome().unwrap_or(Outcome::Draw(DrawReason::Unknown)));
    sleep(Duration::from_secs(60));
}