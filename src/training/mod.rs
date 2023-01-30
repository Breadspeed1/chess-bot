use std::thread::current;
use libm::round;
use owlchess::{Board, Color, DrawReason, Outcome};
use owlchess::Outcome::{Draw, Win};
use crate::player::Agent;

pub struct Tournament {
    players: Vec<Agent>,
    winners: Vec<Agent>,
    current_games: Vec<Game>,
    round: u32
}

pub struct Trainer {
    current: Vec<Agent>,
    size: usize,
    runs: usize,
    mutate_rate: f32,
    genome_length: usize,
    inside_size: usize,
    top: Vec<Agent>
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
            mutate_rate,
            top: Vec::new()
        }
    }

    pub fn run(&mut self) {
        println!("generating players for tournament #{}", self.runs);

        let mut i: usize = 0;
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

        self.runs += 1;

        self.top.push(self.current[0].clone());
    }

    pub fn get_from_recent(&self, i: usize) -> Agent {
        self.top[self.top.len() - i - 1].clone()
    }
}

impl Tournament {
    pub fn new(players: Vec<Agent>) -> Tournament {
        Tournament {
            players,
            winners: Vec::new(),
            current_games: Vec::new(),
            round: 0
        }
    }

    fn set_games(&mut self) {
        if self.players.len() % 2 != 0 {
            self.players.pop();
        }

        for i in (0..self.players.len()).filter(|x| {x % 2 == 0}) {
            self.current_games.push(Game::new(&self.players[i], &self.players[i + 1]));
        }
    }

    pub fn play_through(&mut self) {
        while self.players.len() > 1 {
            self.play_round();
        }
    }

    fn play_round(&mut self) {
        println!("on round {} with {} players", self.round, self.players.len());

        self.current_games.clear();
        self.set_games();
        self.players.clear();
        self.play_games();

        self.round += 1;
    }

    fn play_games(&mut self) {
        for i in 0..self.current_games.len() {
            let res = self.current_games[i].play_through();

            if let Some(x) = res.0 {
                self.winners.insert(0, x.clone());
                self.players.push(x.clone());
            }
        }
    }

    pub fn get_winners(&self) -> Vec<Agent> {
        self.winners.clone()
    }
}

impl Game {
    fn new(white: &Agent, black: &Agent) -> Game {
        Game {
            board: Board::initial(),
            white: white.clone(),
            black: black.clone(),
            moves: 0
        }
    }

    fn play_through(&mut self) -> (Option<&Agent>, u32) {
        while !self.advance() && self.moves < 75 {
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
        match self.side() {
            Color::White => {
                self.board = self.board.make_move(self.white.get_next_move(&self.board)).expect("failed to make move");
            }
            Color::Black => {
                self.board = self.board.make_move(self.black.get_next_move(&self.board)).expect("failed to make move");
            }
        }

        !self.board.has_legal_moves()
    }

    fn side(&self) -> Color {
        self.board.side()
    }
}