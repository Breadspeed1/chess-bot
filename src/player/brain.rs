use std::fmt::{Display, Formatter};
use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};
use libm::tanh;
use rand::{RngCore, thread_rng};
use crate::player::Brain;

pub struct OrganicNNBrain {
    genome: Vec<u8>,
    connections: Vec<Connection>,
    sizes: [usize; 3]
}

struct Connection {
    source_type: usize,
    source_id: usize,
    sink_type: usize,
    sink_id: usize,
    weight: f64
}

impl OrganicNNBrain {
    pub fn new(genome: &[u8], inside_size: usize) -> OrganicNNBrain {
        OrganicNNBrain {
            genome: genome.to_vec(),
            sizes: [64 * 6 + 1, inside_size, 128],
            connections: OrganicNNBrain::gen_connections(genome, inside_size)
        }
    }

    pub fn random(inside_size: usize, conns: usize) -> OrganicNNBrain {
        let mut genome: Vec<u8> = vec![0; conns * 9];
        thread_rng().fill_bytes(&mut genome);

        OrganicNNBrain::new(
            genome.as_slice(),
            inside_size
        )
    }

    fn gen_connections(genome: &[u8], inside_size: usize) -> Vec<Connection> {
        let mut conns: Vec<Connection> = Vec::with_capacity(genome.len() / 8);
        let sizes: [usize; 3] = [64 * 6 + 1, inside_size, 128];

        for b in genome.chunks_exact(9) {
            conns.push(Connection::from_bytes(b, &sizes));
        }

        conns
    }

    fn regen_connections(&mut self) {
        self.connections = OrganicNNBrain::gen_connections(&self.genome[..], self.get_size(1));
    }

    pub fn from_file(path: &str) -> OrganicNNBrain {
        let mut genome: Vec<u8> = Vec::new();
        let mut size: [u8; 8] = [0; 8];
        let mut f = OpenOptions::new().read(true).open(path).expect("cant open file");

        f.read_exact(&mut size).expect("unable to read genome size");
        f.seek(SeekFrom::Start(8)).expect("file too short");
        f.read_to_end(&mut genome).expect("unable to read genome contents");


        OrganicNNBrain::new(genome.as_slice(), usize::from_be_bytes(size))
    }

    pub fn get_mutated(&self, rate: f64) -> OrganicNNBrain {
        let mut out = OrganicNNBrain::new(&self.genome[..], self.get_size(1));
        out.mutate(rate);
        out.regen_connections();

        out
    }
}

impl Brain for OrganicNNBrain {
    fn calc(&self, input: &[f64]) -> Vec<f64> {
        let mut neurons = [
            input.to_vec(),
            vec![0.0; self.get_size(1)],
            vec![0.0; self.get_size(2)]
        ];

        for conn in &self.connections {
            //println!("{}", conn);

            if conn.source_type != 0 {
                neurons[conn.source_type][conn.source_id] = tanh(neurons[conn.source_type][conn.source_id]);
            }

            neurons[conn.sink_type][conn.sink_id] += conn.weight * neurons[conn.source_type][conn.source_id];
        }

        for i in 0..neurons[2].len() {
            neurons[2][i] = tanh(neurons[2][i]);
        }

        //println!("{:?}", neurons[0]);
        //println!("{:?}", self.genome);
        neurons[2][..].to_vec()
    }

    fn write_to(&self, path: &str) {
        let mut f = OpenOptions::new().write(true).create_new(true).open(path).expect("failed to create file");

        f.write_all(self.sizes[1].to_be_bytes().as_slice()).expect("failed to write to file");
        f.write_all(self.genome.as_slice()).expect("failed to write genome");
    }

    fn mutate(&mut self, rate: f64) {
        for i in 0..self.genome.len() {
            if (thread_rng().next_u32() % ((1.0 / rate) as u32)) < 8 {
                self.genome[i] ^= (1 as u8) << (thread_rng().next_u32() % 8) as u8;
            }
        }
    }

    fn get_size(&self, idx: usize) -> usize {
        self.sizes[idx]
    }
}

impl Display for Connection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "({}, {}) -> ({}, {}) [{}]", self.source_type, self.source_id, self.sink_type, self.sink_id, self.weight)
    }
}

impl Connection {
    fn from_bytes(b: &[u8], sizes: &[usize; 3]) -> Connection {
        let sot = (b[0] >> 7) as usize;
        let sit = (((b[0] << 1) >> 7) + 1) as usize;

        Connection {
            source_type: sot as usize,
            sink_type: sit as usize,
            source_id: (u16::from_be_bytes([b[1], b[2]]) % sizes[sot] as u16) as usize,
            sink_id: (u16::from_be_bytes([b[3], b[4]]) % sizes[sit] as u16) as usize,
            weight: (u32::from_be_bytes([b[5], b[6], b[7], b[8]]) as i64 - 2147483648 /*2^31*/) as f64 / 536870912.0 /*2^29*/
        }
    }
}