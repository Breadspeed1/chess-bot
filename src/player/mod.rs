use std::fmt::{Debug, Display, Formatter, Write};
use std::fs::{File, OpenOptions};
use std::io::Read;
use std::{result, u32};
use libm::tanh;
use owlchess::{movegen::legal, Board, Cell, Coord, Move, Piece, Color, MoveKind, Outcome, MoveChain};
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
    life: u64,
    games_won: u32,
    pieces_left: u32,
    opponent_left: u32,
    moves_per_game: f32
}

#[derive(Clone)]
struct Brain {
    genome: Vec<u32>,
    sizes: [usize; 3],
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

    pub fn get_inside_size(&self) -> u32 {
        self.brain.sizes[1] as u32
    }

    pub fn from_file(path: &str) -> Agent {
        let mut genome: Vec<u32> = Vec::new();
        let mut data: Vec<u8> = Vec::new();
        let mut f = OpenOptions::new().read(true).open(path).expect("unable to open agent");
        println!("reading agent data");
        f.read_to_end(&mut data).expect("failed to read file");

        let inside_size = u32::from_be_bytes([data[0], data[1], data[2], data[3]]) as usize;

        let split_data = data.chunks_exact(4);

        println!("processing agent data");
        for x in split_data {
            genome.push(u32::from_be_bytes([x[0], x[1], x[2], x[3]]));
        }

        genome.remove(0);

        println!("creating agent");
        Agent::new(
            genome,
            inside_size
        )
    }

    pub fn new(genome: Vec<u32>, inside_size: usize) -> Agent {
        Agent {
            brain: Brain::new(
                genome,
                inside_size,
            ),
            life: 0,
            games_won: 0,
            pieces_left: 0,
            opponent_left: 0,
            moves_per_game: 0.0
        }
    }

    pub fn track_game(&mut self, ending: &Board, res: Outcome, color: Color, moves: u32) {
        let mut opponent: u32 = 0;
        let mut you: u32 = 0;

        for x in 0..63 as usize {
            let cell = ending.get(Coord::from_index(x));

            if let Some(c) = cell.color() {
                if c == color {
                    you += Self::piece_points(cell.piece().unwrap());
                }
                else {
                    opponent += Self::piece_points(cell.piece().unwrap());
                }
            }
        }

        self.pieces_left += you;
        self.opponent_left += opponent;

        if self.moves_per_game = 0.0 {
            self.moves_per_game = moves as f32;
        }
        else {
            self.moves_per_game = (self.moves_per_game + moves)/2.0;
        }

        if let Some(w) = res.winner() {
            if w == color {
                self.games_won += 1;
            }
        }
    }

    fn piece_points(piece: Piece) -> u32 {
        match piece {
            Piece::Pawn => { 1 }
            Piece::King => { 0 }
            Piece::Knight => { 3 }
            Piece::Bishop => { 3 }
            Piece::Rook => { 5 }
            Piece::Queen => { 9 }
        }
    }

    pub fn get_rating(&self) -> f32 {
        if self.pieces_lost == 0 || self.moves_per_game == 0.0 {
            0.0
        }

        ((self.pieces_left as f32/self.opponent_left as f32) * 10.0 * (self.games_won as f32))/self.moves_per_game
    }

    pub fn make_child(&self, mutation_rate: f32) -> Agent {
        Agent::new(
            self.brain.get_mutated_genome(mutation_rate),
            self.brain.sizes[1]
        )
    }

    pub fn get_data(&self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();

        for x in &self.brain.genome {
            out.append(&mut x.to_be_bytes().to_vec());
        }

        out
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
            connections: Vec::new(),
            sizes: [64 * 6 + 3, inside_size, 128]
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
        let gen = legal::gen_all(board);
        *gen.get(self.calc(board, osc, random) % gen.len()).expect("requested move out of bounds")
    }

    fn set_in(&mut self, data: &Cell, idx: usize) -> usize {
        let offset = match data.piece() {
            None => { return 9999; }
            Some(typ) => {match typ {
                Piece::Pawn => { 0 }
                Piece::King => { 1 }
                Piece::Knight => { 2 }
                Piece::Bishop => { 3 }
                Piece::Rook => { 4 }
                Piece::Queen => { 5 }
            }}
        };

        (idx * 6) + offset
    }

    fn calc(&mut self, board: &Board, osc: f32, random: f32) -> usize {
        let mut neurons: [Vec<f32>; 3] = [vec![0.0; 64 * 6 + 3], vec![0.0; self.sizes[1]], vec![0.0; 128]];

        neurons[0][384] = osc;
        neurons[0][385] = random;
        neurons[0][386] = match board.side() {
            Color::White => { 0.0 }
            Color::Black => { 1.0 }
        };

        for i in 0..64 as usize {
            let t = self.set_in(&board.get(Coord::from_index(i)), i);

            if t < self.sizes[0] {
                neurons[0][t] = 1.0;
            }
        }

        for connection in &self.connections {
            if connection.source_type != 0 {
                neurons[connection.source_type as usize][connection.source_id as usize] = tanh(neurons[connection.source_type as usize][connection.source_id as usize] as f64) as f32;
            }

            neurons[connection.sink_type as usize][connection.sink_id as usize] += connection.weight * neurons[connection.source_type as usize][connection.source_id as usize];
        }

        let mut max: (usize, f32) = (0, neurons[2][0]);

        for i in 0..neurons[2].len() {
            if neurons[2][i] > max.1 {
                max = (i, neurons[2][i]);
            }
        }

        max.0
    }

    fn generate_connections(&mut self) {
        for i in 0..self.genome.len() {
            let segment = self.genome[i];
            let source_type: u8 = binary_util::get_segment(segment, (0b10000000000000000000000000000000 as u32)) as u8;
            let source_id: u32 = binary_util::get_segment(segment, (0b01111111000000000000000000000000 as u32)) as u32 % self.sizes[source_type as usize] as u32;
            let sink_type: u8 = binary_util::get_segment(segment, (0b10000000100000000000000000000000 as u32)) as u8 + 1;
            let sink_id: u32 = binary_util::get_segment(segment, (0b10000000011111110000000000000000 as u32)) as u32 % self.sizes[sink_type as usize] as u32;
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
