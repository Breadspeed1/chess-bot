use libm::tanh;
use owlchess::{movegen::legal, Board, Cell, Coord, Move, Piece};
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

struct Brain {
    genome: Vec<u32>,
    inside_size: usize,
    neurons: [Vec<f32>; 3],
    connections: Vec<Connection>
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

    pub fn get_next_move(&mut self, board: &Board) -> Move {
        self.brain.get_move(board, (self.life % 2) as f32, tanh(thread_rng().next_u32() as f64) as f32)
    }
}

impl Brain {
    fn new(genome: Vec<u32>, inside_size: usize) -> Brain {
        let mut b: Brain = Brain {
            genome,
            inside_size,
            neurons: [
                vec![0.0; (64 * 6) + 2],
                vec![0.0; inside_size],
                vec![0.0; 128]
            ],
            connections: Vec::new()
        };

        b.generate_connections();

        b
    }

    pub fn get_move(&mut self, board: &Board, osc: f32, random: f32) -> Move {
        self.reset();

        for i in 0..64 as usize {
            self.set_in(&board.get(Coord::from_index(i)), i);
        }

        self.calc();

        self.neurons[0][384] = osc;
        self.neurons[0][385] = random;

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
            vec![0.0; (64 * 6) + 2],
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

struct Connection {
    source_type: u8,
    source_id: u32,
    sink_type: u8,
    sink_id: u32,
    weight: f32
}

/*
use libm::tanh;
use rand::Rng;
use serde::{Serialize};

mod binary_util;

#[derive(Serialize)]
pub struct Agent {
    pub genome: Vec<u32>,
    pub pos: (u32, u32),
    brain: Brain,
    rgba: [u8; 4],
    amt_inners: u8
}

impl Clone for Agent {
    fn clone(&self) -> Self {
        Agent {
            pos: self.pos,
            genome: self.genome.clone(),
            brain: self.brain.clone(),
            rgba: self.get_rgba(),
            amt_inners: self.amt_inners
        }
    }
}

impl Agent {
    pub fn new(genome: &Vec<u32>, amt_inners: u8, pos: (u32, u32)) -> Agent {
        Agent {
            pos,
            genome: genome.clone(),
            brain: Brain::from(genome.clone(), amt_inners),
            rgba: Agent::calc_rgba(genome),
            amt_inners
        }
    }

    pub fn set_pos(&mut self, pos: (u32, u32)) {
        self.pos = pos;
    }

    pub fn get_pos(&self) -> (u32, u32) {
        self.pos
    }

    pub fn get_used_inputs(&mut self) -> Vec<usize> {
        self.brain.get_used_inputs()
    }

    pub fn step(&mut self, input: Vec<f32>) -> (i32, i32) {
        self.brain.step(input)
    }

    pub fn produce_child(&mut self, mutation_rate: f32, pos: (u32, u32)) -> Agent {
        let genome = self.mutate_genome(mutation_rate);
        Agent::new(
            &genome,
            self.amt_inners,
            pos
        )
    }

    pub fn get_rgba(&self) -> [u8; 4] {
        self.rgba
    }

    pub fn calc_rgba(genome: &Vec<u32>) -> [u8; 4] {
        let mut av: f32 = 0.0;
        genome.iter().for_each(|x| av += *x as f32);
        let bits = av.to_bits();

        [
            binary_util::get_segment(&bits, 0..=7) as u8,
            binary_util::get_segment(&bits, 8..=15) as u8,
            binary_util::get_segment(&bits, 16..=23) as u8,
            255
        ]
    }

    fn mutate_genome(&mut self, mutation_rate: f32) -> Vec<u32> {
        let mut rng = rand::thread_rng();
        let mut out: Vec<u32> = self.genome.clone();

        for i in 0..out.len() {
            for j in 0..31 {
                if rng.gen_range(0..(1.0/mutation_rate) as i32) == 0 {
                    out[i] = binary_util::flip(&out[i], j)
                }
            }
        }

        out
    }
}

#[derive(Serialize)]
struct Brain {
    genome: Vec<u32>,
    move_activation: f32,
    used_input_ids: Vec<usize>,
    connections: Vec<Connection>,
    neurons: Vec<Vec<f32>>,
    move_vec: Vec<(i32, i32)>
}

impl Clone for Brain {
    fn clone(&self) -> Self {
        Brain {
            genome: self.genome.clone(),
            move_activation: self.move_activation,
            used_input_ids: self.used_input_ids.clone(),
            connections: self.connections.clone(),
            neurons: self.neurons.clone(),
            move_vec: self.move_vec.clone()
        }
    }
}

impl Brain {
    pub fn from(genome: Vec<u32>, amt_inners: u8) -> Brain {
        let mut out: Brain = Brain {
            genome,
            move_activation: 0.0,
            used_input_ids: Vec::new(),
            connections: Vec::new(),
            neurons: vec![
                vec![0.0; 14],
                vec![0.0; amt_inners as usize],
                vec![0.0; 5]
            ],
            move_vec: vec![
                (0, 1),
                (0, -1),
                (1, 0),
                (-1, 0)
            ]
        };

        out.generate_connections();

        out
    }

    fn step(&mut self, input: Vec<f32>) -> (i32, i32) {
        self.reset_all();
        self.neurons[0] = input;
        self.calculate_all();

        let mut request = (0, 0);

        if self.neurons[2][0] > self.move_activation {
            let mut rand = rand::thread_rng();
            request = (request.0 + rand.gen_range(-1..1), request.1 + rand.gen_range(-1..1));
        }

        let move_vec: Vec<(i32, i32)> = vec![
            (0, 1),
            (0, -1),
            (1, 0),
            (-1, 0)
        ];

        for i in 1..self.neurons[2].len() {
            if self.neurons[2][i] > self.move_activation {
                request = (request.0 + move_vec[i - 1].0, request.1 + move_vec[i - 1].1);
            }
        }

        request.clamp((-1, -1), (1, 1))
    }

    fn get_used_inputs(&mut self) -> Vec<usize> {
        self.used_input_ids.clone()
    }

    fn reset_all(&mut self) {
        for i in 1..self.neurons.len() {
            for j in 0..self.neurons[i].len() {
                self.neurons[i][j] = 0.0;
            }
        }
    }

    fn calculate_all(&mut self) {
        for i in 0..self.connections.len() {
            self.calculate(i);
        }

        for i in 0..self.neurons[2].len() {
            self.neurons[2][i] = tanh(self.neurons[2][i] as f64) as f32
        }
    }

    fn calculate(&mut self, index: usize) {
        let connection: &Connection = &self.connections[index];

        if connection.source_type != 0 {
            self.neurons[connection.source_type as usize][connection.source_id as usize] = libm::tanh(self.neurons[connection.source_type as usize][connection.source_id as usize] as f64) as f32;
        }

        self.neurons[connection.sink_type as usize][connection.sink_id as usize] += connection.weight * self.neurons[connection.source_type as usize][connection.source_id as usize];
    }

    fn generate_connections(&mut self) {
        for i in 0..self.genome.len() {
            self.generate_connection_from_genome_segment(i);
        }

        self.connections.sort_by(|a, b| a.sink_id.cmp(&b.sink_id));
    }

    fn generate_connection_from_genome_segment(&mut self, index: usize) {
        let dec: u32 = self.genome[index];

        let source_type: u8 = binary_util::get_segment(&dec, /*&(0b10000000000000000000000000000000 as u32)*/ 0..=0) as u8;
        let source_id: u8 = binary_util::get_segment(&dec, /*&(0b01111111000000000000000000000000 as u32)*/ 1..=6) as u8 % self.neurons[source_type as usize].len() as u8;
        let sink_type: u8 = binary_util::get_segment(&dec, /*&(0b10000000100000000000000000000000 as u32)*/ 7..=7) as u8 + 1;
        let sink_id: u8 = binary_util::get_segment(&dec, /*&(0b10000000011111110000000000000000 as u32)*/8..=15) as u8 % self.neurons[sink_type as usize].len() as u8;
        //println!("{}", self.neurons[sink_type as usize].len());
        let weight: f32;

        if binary_util::get_segment(&dec, /*&(0b00000000000000001000000000000000 as u32)*/ 16..=16) == 1 {
            weight = binary_util::get_segment(&dec, /*&(0b00000000000000000111111111111111 as u32)*/ 17..=31) as f32 / 16000.0;
        }
        else {
            weight = binary_util::get_segment(&dec, /*&(0b00000000000000000111111111111111 as u32)*/ 17..=31) as f32 / -16000.0;
        }

        if sink_type == 0 {
            if sink_id > 6 {
                self.used_input_ids.push(sink_id as usize);
            }
        }


        //println!("{}-{} {}-{} {}", source_type, source_id, sink_type, sink_id, weight);

        self.connections.push(Connection{
            source_type,
            source_id,
            sink_type,
            sink_id,
            weight
        })
    }
}

#[derive(Serialize)]
struct Connection {
    source_type: u8,
    source_id: u8,
    sink_type: u8,
    sink_id: u8,
    weight: f32
}

impl Clone for Connection {
    fn clone(&self) -> Self {
        Connection {
            source_type: self.source_type,
            source_id: self.source_id,
            sink_type: self.sink_type,
            sink_id: self.sink_id,
            weight: self.weight
        }
    }
}
*/
