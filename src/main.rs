mod player;
mod training;

use std::env;
use std::fmt::format;
use std::thread::sleep;
use std::time::Duration;
use clearscreen::clear;
use owlchess::{Board, Color, DrawReason, Make, MoveChain, Outcome};
use owlchess::board::PrettyStyle;
use rand::thread_rng;
use crate::player::Agent;
use crate::training::{Tournament, Trainer};

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    match args[1].as_str() {
        "fight" => {
            fight(
                Agent::from_file(format!("out/{}.agent", args[2]).as_str()),
                Agent::from_file(format!("out/{}.agent", args[3]).as_str())
            )
        },
        "train" => {
            if args.len() == 4 {

            }
            else {
                let mut trainer: Trainer = Trainer::new(4096, 512, 64, 0.001);

                for _ in 0..100000 {
                    trainer.run();
                }
            }
        },
        _ => panic!("action not specified! (fight/train)")
    }
}

fn fight(o: Agent, t: Agent) {
    let mut one = o;
    let mut two = t;

    let mut board: Board = Board::initial();
    let mut game: MoveChain = MoveChain::default();

    println!("starting game");

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
    }
    println!("{}", game.uci());
    sleep(Duration::from_secs(60));
}