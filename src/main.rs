mod player;
mod training;
pub mod game;

use std::env;
use std::fmt::format;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use owlchess::{Board, Color, MoveChain};
use crate::player::agent::OrganicAgent;
use crate::player::brain::OrganicNNBrain;
use crate::player::Player;
use crate::training::{OrganicTrainer, OrganicTrainerSettings};
use crate::training::tournament::play_organic_game;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    match args[1].as_str() {
        "fight" => {
            fight(
                OrganicAgent::new(&Arc::new(OrganicNNBrain::from_file(format!("out/{}.agent", args[2]).as_str()))),
                OrganicAgent::new(&Arc::new(OrganicNNBrain::from_file(format!("out/{}.agent", args[3]).as_str())))
            )
        },
        "train" => {
            let settings: OrganicTrainerSettings = OrganicTrainerSettings {
                connection_count: 1024,
                inner_neuron_count: 128,
                mutation_rate: 0.001,
                pool_size: 64,
                save_path: "out".to_string(),
                reproduction_sample: 4,
            };

            let mut trainer: OrganicTrainer = OrganicTrainer::new(settings);

            trainer.init();

            loop {
                trainer.advance_generation();
            }
        },
        _ => panic!("action not specified! (fight/train)")
    }
}

fn fight(o: OrganicAgent, t: OrganicAgent) {
    play_organic_game(&o, &t, true);
}