use std::fs::OpenOptions;
use std::io::Write;
use std::ops::Add;
use std::sync::Arc;
use crate::player::agent::OrganicAgent;
use crate::player::brain::OrganicNNBrain;
use crate::player::{Brain, Player};
use crate::training::tournament::rank_organic_players;

pub mod tournament;

pub struct OrganicTrainer {
    settings: OrganicTrainerSettings,
    generation: u32,
    pool: Vec<Arc<OrganicNNBrain>>
}

pub struct OrganicTrainerSettings {
    pub connection_count: usize,
    pub inner_neuron_count: usize,
    pub mutation_rate: f64,
    pub pool_size: usize,
    pub save_path: String,
    pub reproduction_sample: usize
}

impl OrganicTrainer {
    pub fn new(settings: OrganicTrainerSettings) -> OrganicTrainer {
        let vec = Vec::with_capacity(settings.pool_size);
        OrganicTrainer {
            settings,
            generation: 0,
            pool: vec
        }
    }

    pub fn init(&mut self) {
        println!("creating initial pool for gen {}", self.generation);
        for _ in 0..self.settings.pool_size {
            self.pool.push(Arc::new(OrganicNNBrain::random(self.settings.inner_neuron_count, self.settings.connection_count)));
        }

        OpenOptions::new().write(true).create(true).append(true).open(format!("{}/stats.stat", self.settings.save_path)).expect("failed to create file");

        println!("finished creating initial pool");
    }

    pub fn advance_generation(&mut self) {
        let mut players: Vec<OrganicAgent> = Vec::with_capacity(self.settings.pool_size);

        println!("creating agents");

        for i in 0..self.pool.len() {
            players.push(OrganicAgent::new(&self.pool[i]));
        }

        println!("playing games");

        rank_organic_players(&mut players);

        self.pool = Vec::with_capacity(self.settings.pool_size);

        println!("generating new pool");
        let mut i: usize = 0;
        while self.pool.len() < self.settings.pool_size {
            self.pool.push(Arc::new(players[i % self.settings.reproduction_sample].get_brain().get_mutated(self.settings.mutation_rate)));

            i += 1;
        }

        println!("saving agent");
        if !self.settings.save_path.is_empty() {
            players[0].get_brain().write_to(format!("{}/{}.agent", self.settings.save_path, self.generation).as_str());
            OpenOptions::new().write(true).append(true).open(format!("{}/stats.stat", self.settings.save_path)).expect("failed to create file").write_all(&players[0].get_rating().to_be_bytes()).expect("failed to write to stat file");

        }

        println!("top rating for generation {}: {}", self.generation, players[0].get_rating());

        self.generation += 1;
    }
}