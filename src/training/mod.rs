use std::fs::{File, OpenOptions};
use std::io::Write;
use std::ops::Add;
use std::path::Path;
use std::thread::current;
use libm::round;
use owlchess::{Board, Color, DrawReason, Outcome};
use owlchess::Outcome::{Draw, Win};
use crate::player::Agent;

pub struct Tournament {
    players: Vec<Agent>
}

pub struct Trainer {
    current: Vec<Agent>,
    size: usize,
    runs: usize,
    mutate_rate: f32,
    genome_length: usize,
    inside_size: usize
}

struct Game {
    board: Board,
    white: Agent,
    black: Agent,
    moves: u32
}

impl Trainer {
    pub fn new(size: usize, genome_length: usize, inside_size: usize, mutate_rate: f32) -> Trainer {
        let mut players: Vec<Agent> = Vec::new();

        println!("generating initial players");

        for _ in 0..size {
            players.push(Agent::random(genome_length, inside_size));
        }

        Trainer {
            current: players,
            size,
            runs: 0,
            genome_length,
            inside_size,
            mutate_rate
        }
    }

    pub fn run(&mut self) {
        println!("generating players for tournament #{}", self.runs);

        /*let mut i: usize = 0;
        while self.current.len() < self.size {
            self.current.push(self.current[i % self.current.len()].make_child(self.mutate_rate));
        }

        println!("starting tournament #{}", self.runs);
        let mut t = Tournament::new(self.current.clone());

        t.play_through();
        let x = t.get_winners();

        if x.len() > 0 {
            self.current = t.get_winners();
        }
*/
        self.runs += 1;
    }
}

impl Tournament {
    pub fn new(players: Vec<Agent>) -> Tournament {
        Tournament {
            players
        }
    }

    pub fn play_through(&mut self) {
        for i in 0..self.players.len() {
            for j in 0..self.players.len() {
                if j != i {
                    let black = &self.players[i];
                    let white = &self.players[j];
                }
            }
        }
    }

    /*pub fn get_top_x() -> Vec<&Agent> {
        Vec::new()
    }*/
}

impl Game {
    fn new(white: &Agent, black: &Agent) -> Game {
        Game {
            board: Board::initial(),
            white: Agent::random(0, 0),
            black: Agent::random(0, 0),
            moves: 0
        }
    }

    fn play_through(&mut self) -> (Option<&Agent>, u32) {
        while !self.advance() && self.moves < 150 {
            self.moves += 1;
        }

        match self.board.calc_outcome().unwrap_or(Draw(DrawReason::Moves75)).winner() {
            None => { (None, self.moves) },
            Some(winner) => {
                match winner {
                    Color::White => {
                        (Some(&self.white), self.moves)
                    }
                    Color::Black => {
                        (Some(&self.black), self.moves)
                    }
                }
            },
        }
    }

    fn advance(&mut self) -> bool {
        /*match self.side() {
            Color::White => {
                self.board = self.board.make_move(self.white.get_next_move(&self.board)).expect("failed to make move");
            }
            Color::Black => {
                self.board = self.board.make_move(self.black.get_next_move(&self.board)).expect("failed to make move");
            }
        }*/

        !self.board.has_legal_moves()
    }

    fn side(&self) -> Color {
        self.board.side()
    }
}