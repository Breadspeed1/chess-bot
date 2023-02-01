use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::sync::Arc;
use owlchess::{Board, Color, Coord, Move};
use owlchess::movegen::legal;
use crate::game::get_id;
use crate::player::{Brain, Player};
use crate::player::brain::OrganicNNBrain;

pub struct OrganicAgent {
    brain: Arc<OrganicNNBrain>,
    rating: f64
}

impl OrganicAgent {
    pub fn new(brain: &Arc<OrganicNNBrain>) -> OrganicAgent {
        OrganicAgent {
            brain: brain.clone(),
            rating: 0.0
        }
    }

    fn set_in(&self, board: &Board, color: &Color) -> Vec<f64> {
        let mut out: Vec<f64> = vec![0.0; 385];

        for i in 0..63 as usize {
            if let Some(p) = board.get(Coord::from_index(i)).piece() {
                out[(i * 6) + get_id(p)] = 1.0;
            }
        }

        if *color == Color::Black {
            out[384] = 1.0;
        }

        out
    }

    fn output_to_move(output: &[f64], board: &Board) -> Move {
        let mut max: usize = 0;

        for i in 0..output.len() {
            if output[i] > output[max] {
                max = i;
            }
        }

        let gen = legal::gen_all(board);
        *gen.get(max % gen.len()).unwrap()
    }
}

impl Eq for OrganicAgent {}

impl PartialEq<Self> for OrganicAgent {
    fn eq(&self, other: &Self) -> bool {
        self.rating == other.rating
    }
}

impl PartialOrd<Self> for OrganicAgent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.rating.partial_cmp(&other.rating)
    }
}

impl Ord for OrganicAgent {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rating.total_cmp(&other.rating)
    }
}

impl Display for OrganicAgent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.get_rating())
    }
}

impl Debug for OrganicAgent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.get_rating())
    }
}

impl Player<OrganicNNBrain> for OrganicAgent {
    fn get_move(&self, board: &Board, color: &Color) -> Move {
        OrganicAgent::output_to_move(
            &self.brain.calc(
                &self.set_in(
                    board,
                    color)[..])[..],
            board
        )
    }

    fn get_rating(&self) -> f64 {
        self.rating
    }

    fn set_rating(&mut self, rating: f64) {
        self.rating = rating
    }

    fn get_brain(&self) -> &OrganicNNBrain {
        &self.brain
    }
}