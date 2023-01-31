use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{SeekFrom, Write};
use libm::tanh;
use owlchess::{movegen::legal, Board, Cell, Coord, Move, Piece, Color, MoveKind, Outcome, MoveChain};
use rand::{Rng, RngCore, thread_rng};
use crate::game::get_id;

mod binary_util;

pub fn random_genome(size: usize) -> Vec<u32> {
    let mut out: Vec<u32> = Vec::new();

    for _ in 0..size {
        out.push(thread_rng().next_u32());
    }

    out
}

pub struct Agent<> {
    genome: Vec<u8>,
    connections: Vec<Connection>,
    sizes: [usize; 3]
}

impl PartialEq<Self> for Agent {
    fn eq(&self, other: &Self) -> bool {
        self.genome == other.genome
    }
}

impl Agent {
    pub fn new(genome: &[u8], inside_size: usize) -> Agent {
        Agent {
            genome: genome.to_vec(),
            sizes: [64 * 6 + 1, inside_size, 128],
            connections: Agent::gen_connections(genome, inside_size)
        }
    }

    pub fn from_file(path: &str) -> Agent {
        let mut genome: Vec<u8> = Vec::new();
        let mut size: [u8; 8] = [0; 8];
        let mut f = OpenOptions::new().read(true).open(path).expect("cant open file");

        f.read_exact(&mut size).expect("unable to read genome size");
        f.seek(SeekFrom::Start(8)).expect("file too short");
        f.read_to_end(&mut genome).expect("unable to read genome contents");


        Agent::new(genome.as_slice(), usize::from_be_bytes(size))
    }

    pub fn write_to(&self, path: &str) {
        let mut f = OpenOptions::new().write(true).create_new(true).open(path).expect("failed to create file");

        f.write_all(self.sizes[1].to_be_bytes().as_slice()).expect("failed to write to file");
        f.write_all(self.genome.as_slice()).expect("fialed to write genome");
    }

    pub fn random(inside_size: usize, conns: usize) -> Agent {
        let mut genome: Vec<u8> = Vec::with_capacity(conns * 9);
        thread_rng().fill_bytes(&mut genome);

        Agent::new(
            genome.as_slice(),
            inside_size
        )
    }

    pub fn from(agent: &Agent, mut_rate: f32) -> Agent {
        Agent::new(
            Agent::mutate(&agent.genome, mut_rate).as_slice(),
            agent.sizes[1]
        )
    }

    fn mutate(genome: &Vec<u8>, rate: f32) -> Vec<u8> {
        let mut out = genome.clone();

        for i in 0..genome.len() {
            if (thread_rng().next_u32() % ((1.0 / rate) as u32)) < 8 {
                out[i] ^= (1 as u8) << (thread_rng().next_u32() % 8) as u8;
            }
        }

        out
    }

    fn gen_connections(genome: &[u8], inside_size: usize) -> Vec<Connection> {
        let mut conns: Vec<Connection> = Vec::with_capacity(genome.len() / 8);
        let sizes: [usize; 3] = [64 * 6 + 1, inside_size, 128];

        for b in genome.chunks_exact(9) {
            conns.push(Connection::from_bytes(b, &sizes));
        }

        conns
    }

    pub fn get_move(&self, board: &Board, color: &Color) -> Move {
        let mut invec = vec![0.0; self.sizes[0]];
        let mut outvec = vec![0.0; self.sizes[2]];

        let input: &mut [f64] = invec.as_mut_slice();
        let output: &mut [f64] = outvec.as_mut_slice();

        Agent::set_in(board, color, input);

        self.calc(input, output);

        Agent::output_to_move(output, board)
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

    fn set_in(board: &Board, color: &Color, input: &mut [f64]) {
        for i in 0..63 as usize {
            if let Some(p) = board.get(Coord::from_index(i)).piece() {
                input[(i * 6) + get_id(p)] = 1.0;
            }
        }

        if *color == Color::Black {
            input[384] = 1.0;
        }
    }

    fn calc(&self, input: &mut [f64], output: &mut [f64]) {
        let mut inner = vec![0.0; self.sizes[1]];
        let neurons = [input, inner.as_mut_slice(), output];

        for conn in &self.connections {
            if conn.source_type != 0 {
                neurons[conn.source_type][conn.source_id] = tanh(neurons[conn.source_type][conn.source_id]);
            }

            neurons[conn.sink_type][conn.sink_id] += conn.weight * neurons[conn.source_type][conn.source_id];
        }

        for i in 0..neurons[2].len() {
            neurons[2][i] = tanh(neurons[2][i]);
        }
    }
}

impl Connection {
    fn from_bytes(b: &[u8], sizes: &[usize; 3]) -> Connection {
        let sot = (b[0] >> 7) as usize;
        let sit = ((b[0] << 1) >> 7) as usize;

        Connection {
            source_type: sot as usize,
            sink_type: sit as usize,
            source_id: (u16::from_be_bytes([b[1], b[2]]) % sizes[sot] as u16) as usize,
            sink_id: (u16::from_be_bytes([b[3], b[4]]) % sizes[sit] as u16) as usize,
            weight: (u32::from_be_bytes([b[5], b[6], b[7], b[8]]) as i64 - 2147483648 /*2^31*/) as f64 / 536870912.0 /*2^29*/
        }
    }
}

#[derive(Clone)]
struct Connection {
    source_type: usize,
    source_id: usize,
    sink_type: usize,
    sink_id: usize,
    weight: f64
}
