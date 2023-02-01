use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use owlchess::{Board, Color, Move, MoveChain};
use crate::game::get_color_points;
use crate::player::{Brain, Player};
use crate::player::agent::OrganicAgent;

pub fn rank_organic_players(players: &mut Vec<OrganicAgent>) {
    let mut handlers: Vec<JoinHandle<(usize, Vec<(f64, f64)>)>> = Vec::new();

    for i in 0..players.len() {
        let x = i;
        let slice = players.clone();

        handlers.push(thread::spawn(move || {
            let mut ratings: Vec<(f64, f64)> = Vec::new();

            for j in 0..slice.len() {
                if i != j {
                    let res = play_organic_game(&slice[i], &slice[j], false);

                    ratings.push((res.0, res.0));

                    //println!("white score: {}, black score: {}", players[i].get_rating(), players[j].get_rating());
                }
                else {
                    ratings.push((0.0, 0.0));
                }
            }

            (x, ratings)
        }));
    }

    let mut scores: Vec<(usize, Vec<(f64, f64)>)> = Vec::new();

    for x in handlers {
        let y = x.join().expect("failed to join thread");
        scores.push(y);
    }

    scores.sort_by(|x, y| {x.0.cmp(&y.0)});

    for x in 0..63 {
        for y in 0..63 {
            let white_rating = players[x].get_rating();
            let black_rating = players[y].get_rating();

            players[x].set_rating(white_rating + scores[x/8].1[y].0);
            players[y].set_rating(black_rating + scores[x/8].1[y].1);
        }
    }

    players.sort();
    players.reverse();

    //println!("{:?}", players);
}

pub fn play_organic_game(white: &OrganicAgent, black: &OrganicAgent, print_uci: bool) -> (f64, f64) {
    let mut board: Board = Board::initial();
    let mut game: MoveChain = MoveChain::new(Board::initial());

    while game.calc_outcome() == None {
        let n_move: Move = match board.side() {
            Color::White => {
                white.get_move(&board, &Color::White)
            }
            Color::Black => {
                black.get_move(&board, &Color::Black)
            }
        };

        board = board.make_move(n_move).unwrap();
        game.push(n_move).unwrap();
    }

    if print_uci {
        println!("{}", game.uci());
    }

    calc_score(&board, game.len(), game.calc_outcome().unwrap().winner())
}

fn calc_score(board: &Board, moves: usize, winner: Option<Color>) -> (f64, f64) {
    let mut white_points = get_color_points(board, &Color::White).clamp(1, usize::MAX);
    let mut black_points = get_color_points(board, &Color::Black).clamp(1, usize::MAX);

    let mut out = match winner {
        None => { (0.5, 0.5) }
        Some(color) => {
            match color {
                Color::White => { (1.0, -1.0) }
                Color::Black => { (-1.0, 1.0) }
            }
        }
    };


    if out.0 > 0.0 {
        out.0 *= white_points as f64/black_points as f64;
    }
    else {
        out.0 *= black_points as f64/white_points as f64;
    }

    if out.1 > 0.0 {
        out.1 *= black_points as f64/white_points as f64;
    }
    else {
        out.1 *= white_points as f64/black_points as f64;
    }
    //(out.0 / (moves as f64 / 50.0), out.1 / (moves as f64 / 50.0))
    out
}