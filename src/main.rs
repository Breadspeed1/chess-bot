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
    let mut trainer: Trainer = Trainer::new(4096, 512, 64, 0.001);

    for _ in 0..100000 {
        trainer.run();
    }
}