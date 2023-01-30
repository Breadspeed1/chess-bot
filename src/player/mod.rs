use std::fmt::{Debug, Display, Formatter, Write};
use libm::tanh;
use owlchess::{movegen::legal, Board, Cell, Coord, Move, Piece, Color};
use rand::{RngCore, thread_rng};

mod binary_util;

pub fn random_genome(size: usize) -> Vec<u32> {
    let mut out: Vec<u32> = Vec::new();

    for _ in 0..size {
        out.push(thread_rng().next_u32());
    }

    out
}

#[derive(Clone)]
pub struct Agent {
    brain: Brain,
    life: u64
}

#[derive(Clone)]
struct Brain {
    genome: Vec<u32>,
    inside_size: usize,
    neurons: [Vec<f32>; 3],
    connections: Vec<Connection>
}

impl Display for Agent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Agent")
    }
}

impl Debug for Agent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Agent")
    }
}

impl Agent {
    pub fn random(genome_length: usize, inside_size: usize) -> Agent {
        Agent::new(
            random_genome(genome_length),
            inside_size
        )
    }

    pub fn new(genome: Vec<u32>, inside_size: usize) -> Agent {
        Agent {
            brain: Brain::new(
                genome,
                inside_size
            ),
            life: 0
        }
    }

    pub fn make_child(&self, mutation_rate: f32) -> Agent {
        Agent::new(
            self.brain.get_mutated_genome(mutation_rate),
            self.brain.inside_size
        )
    }

    pub fn get_next_move(&mut self, board: &Board) -> Move {
        self.life += 1;
        self.brain.get_move(board, (self.life % 2) as f32, tanh(thread_rng().next_u32() as f64) as f32)
    }
}

impl Brain {
    fn new(genome: Vec<u32>, inside_size: usize) -> Brain {
        let mut b: Brain = Brain {
            genome,
            inside_size,
            neurons: [
                vec![0.0; (64 * 6) + 3],
                vec![0.0; inside_size],
                vec![0.0; 128]
            ],
            connections: Vec::new()
        };

        b.generate_connections();

        b
    }

    fn get_mutated_genome(&self, mutation_rate: f32) -> Vec<u32> {
        let mut out: Vec<u32> = Vec::new();

        for x in &self.genome {
            let mut c = *x;

            for j in 0..32 {
                if thread_rng().next_u32() % ((1.0/mutation_rate) as u32) == 1 {
                    c ^= 1 << j;
                }
            }

            out.push(c);
        }

        out
    }

    pub fn get_move(&mut self, board: &Board, osc: f32, random: f32) -> Move {
        self.reset();

        for i in 0..64 as usize {
            self.set_in(&board.get(Coord::from_index(i)), i);
        }

        self.calc();

        self.neurons[0][384] = osc;
        self.neurons[0][385] = random;
        self.neurons[0][386] = match board.side() {
            Color::White => { 0.0 }
            Color::Black => { 1.0 }
        };

        let mut max: (usize, f32) = (0, self.neurons[2][0]);

        for i in 0..self.neurons[2].len() {
            if self.neurons[2][i] > max.1 {
                max = (i, self.neurons[2][i]);
            }
        }

        let gen = legal::gen_all(board);
        *gen.get(max.0 % gen.len()).expect("requested move out of bounds")
    }

    fn set_in(&mut self, data: &Cell, idx: usize) {
        let offset = match data.piece() {
            None => { return; }
            Some(typ) => {match typ {
                Piece::Pawn => { 0 }
                Piece::King => { 1 }
                Piece::Knight => { 2 }
                Piece::Bishop => { 3 }
                Piece::Rook => { 4 }
                Piece::Queen => { 5 }
            }}
        };

        self.neurons[0][(idx * 6) + offset] = 1.0;
    }

    fn reset(&mut self) {
        self.neurons = [
            vec![0.0; (64 * 6) + 3],
            vec![0.0; self.inside_size],
            vec![0.0; 128]
        ];
    }

    fn calc(&mut self) {
        for connection in &self.connections {
            if connection.source_type != 0 {
                self.neurons[connection.source_type as usize][connection.source_id as usize] = tanh(self.neurons[connection.source_type as usize][connection.source_id as usize] as f64) as f32;
            }

            self.neurons[connection.sink_type as usize][connection.sink_id as usize] += connection.weight * self.neurons[connection.source_type as usize][connection.source_id as usize];
        }
    }

    fn generate_connections(&mut self) {
        for i in 0..self.genome.len() {
            let segment = self.genome[i];
            let source_type: u8 = binary_util::get_segment(segment, (0b10000000000000000000000000000000 as u32)) as u8;
            let source_id: u32 = binary_util::get_segment(segment, (0b01111111000000000000000000000000 as u32)) as u32 % self.neurons[source_type as usize].len() as u32;
            let sink_type: u8 = binary_util::get_segment(segment, (0b10000000100000000000000000000000 as u32)) as u8 + 1;
            let sink_id: u32 = binary_util::get_segment(segment, (0b10000000011111110000000000000000 as u32)) as u32 % self.neurons[sink_type as usize].len() as u32;
            let weight: f32;

            if binary_util::get_segment(segment, (0b00000000000000001000000000000000 as u32)) == 1 {
                weight = binary_util::get_segment(segment, (0b00000000000000000111111111111111 as u32)) as f32 / 16000.0;
            }
            else {
                weight = binary_util::get_segment(segment, (0b00000000000000000111111111111111 as u32)) as f32 / -16000.0;
            }

            if source_type == 0 {
                self.connections.insert(0, Connection{
                    source_type,
                    source_id,
                    sink_type,
                    sink_id,
                    weight
                })
            }
            else {
                self.connections.push(Connection{
                    source_type,
                    source_id,
                    sink_type,
                    sink_id,
                    weight
                })
            }
        }
    }
}

#[derive(Clone)]
struct Connection {
    source_type: u8,
    source_id: u32,
    sink_type: u8,
    sink_id: u32,
    weight: f32
}
