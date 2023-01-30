mod player;

use std::thread::sleep;
use std::time::Duration;
use clearscreen::clear;
use owlchess::{Board, Color, DrawReason, Make, Outcome};
use owlchess::board::PrettyStyle;
use crate::player::Agent;

fn main() {
    let mut agent1: Agent = Agent::new(player::random_genome(8192), 2048);
    let mut agent2: Agent = Agent::new(player::random_genome(8192), 2048);

    let mut game: Board = Board::initial();
    let mut moves: u32 = 0;

    while game.has_legal_moves() {
        match game.side() {
            Color::White => {game = game.make_move(agent1.get_next_move(&game)).expect("failed to make move");}
            Color::Black => {game = game.make_move(agent2.get_next_move(&game)).expect("failed to make move");}
        }

        clear().expect("failed to clear screen");
        println!("{}", game.pretty(PrettyStyle::Ascii));
        sleep(Duration::from_millis(20));
    }

    println!("{}", game.calc_outcome().unwrap_or(Outcome::Draw(DrawReason::Unknown)));
    sleep(Duration::from_secs(60));
}