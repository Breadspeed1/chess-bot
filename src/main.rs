mod player;
mod training;

use std::thread::sleep;
use std::time::Duration;
use clearscreen::clear;
use owlchess::{Board, Color, DrawReason, Make, MoveChain, Outcome};
use owlchess::board::PrettyStyle;
use rand::thread_rng;
use crate::player::Agent;
use crate::training::{Tournament, Trainer};

fn main() {
    let mut trainer: Trainer = Trainer::new(4096, 512, 64, 0.001);

    for _ in 0..100000 {
        trainer.run();
    }

    let mut one = trainer.get_from_recent(0);
    let mut two = trainer.get_from_recent(1);

    let mut board: Board = Board::initial();
    let mut game: MoveChain = MoveChain::default();

    while board.has_legal_moves() {
        match board.side() {
            Color::White => {
                let x = one.get_next_move(&board);
                board = board.make_move(x.clone()).expect("failed to make move");
                game.push(x).expect("failed to make move");
            }
            Color::Black => {
                let x = two.get_next_move(&board);
                board = board.make_move(x.clone()).expect("failed to make move");
                game.push(x).expect("failed to make move");
            }
        }

        /*clear().expect("error clearing screen");
        println!("{}", board.pretty(PrettyStyle::Ascii));
        sleep(Duration::from_millis(500));*/
    }

    //println!("{}", board.calc_outcome().unwrap_or(Outcome::Draw(DrawReason::Unknown)));
    println!("{}", game.uci());
    sleep(Duration::from_secs(60));
}