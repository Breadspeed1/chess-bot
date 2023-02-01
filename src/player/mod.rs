use std::cmp::Ordering;
use owlchess::{Board, Color, Move};

pub mod brain;
pub mod agent;

pub trait Brain {
    fn calc(&self, input: &[f64]) -> Vec<f64>;
    fn write_to(&self, path: &str);
    fn mutate(&mut self, rate: f64);
    fn get_size(&self, idx: usize) -> usize;
}

pub trait Player<T> {
    fn get_move(&self, board: &Board, color: &Color) -> Move;
    fn get_rating(&self) -> f64;
    fn set_rating(&mut self, rating: f64);
    fn get_brain(&self) -> &T;
}

impl PartialEq<Self> for dyn Player<()> {
    fn eq(&self, other: &Self) -> bool {
        other.get_rating() == self.get_rating()
    }
}

impl PartialOrd for dyn Player<()> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.get_rating().partial_cmp(&other.get_rating())
    }
}