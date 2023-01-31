mod player;
mod training;
pub mod game;

use std::env;
use std::fmt::format;
use std::thread::sleep;
use std::time::Duration;
use owlchess::{Board, Color, MoveChain};
use crate::player::Agent;
use crate::training::{Tournament, Trainer};

fn main() {
    let a: Agent = Agent::random(256, 1024);
    a.write_to("out/1.agent");

    assert!(a == Agent::from_file("out/1.agent"));

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
                let mut trainer: Trainer = Trainer::new(4096, 1024, 128, 0.001);

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
                let x = one.get_move(&board, &Color::White);
                board = board.make_move(x.clone()).expect("failed to make move");
                game.push(x).expect("failed to make move");
            }
            Color::Black => {
                let x = two.get_move(&board, &Color::Black);
                board = board.make_move(x.clone()).expect("failed to make move");
                game.push(x).expect("failed to make move");
            }
        }
    }
    println!("{}", game.uci());
    sleep(Duration::from_secs(60));
}