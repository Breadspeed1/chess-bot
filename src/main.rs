mod player;
mod training;

use std::thread::sleep;
use std::time::Duration;
use clearscreen::clear;
use owlchess::{Board, Color, DrawReason, Make, Outcome};
use owlchess::board::PrettyStyle;
use crate::player::Agent;
use crate::training::Tournament;

fn main() {
    let mut players: Vec<Agent> = Vec::new();

    for x in 0..8192 {
        players.push(Agent::random(8192, 256));
        if x % 100 == 0 {
            println!("on agent {}", x);
        }
    }

    let mut tournament: Tournament = Tournament::new(players);

    tournament.play_through();

    println!("{:?}", tournament.get_winners());

    /*let mut agent1: Agent = Agent::new(player::random_genome(8192), 256);
    let mut agent2: Agent = Agent::new(player::random_genome(8192), 256);

    let mut game: Board = Board::initial();
    let mut moves: u32 = 0;

    while moves < 75 && game.has_legal_moves() {
        match game.side() {
            Color::White => {game = game.make_move(agent1.get_next_move(&game)).expect("failed to make move");}
            Color::Black => {game = game.make_move(agent2.get_next_move(&game)).expect("failed to make move");}
        }

        clear().expect("failed to clear screen");
        println!("{}", game.pretty(PrettyStyle::Ascii));
        sleep(Duration::from_millis(20));

        moves += 1;
    }

    println!("{}", game.calc_outcome().unwrap_or(Outcome::Draw(DrawReason::Moves75)));
    sleep(Duration::from_secs(60));*/
}