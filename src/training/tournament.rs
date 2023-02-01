use std::sync::Arc;
use owlchess::{Board, Color, Move, MoveChain};
use crate::game::get_color_points;
use crate::player::{Brain, Player};
use crate::player::agent::OrganicAgent;

pub fn rank_organic_players(players: &mut Vec<OrganicAgent>) {
    for i in 0..players.len() {
        for j in 0..players.len() {
            if i != j {
                let res = play_organic_game(&players[i], &players[j], false);
                let white_rating = players[i].get_rating();
                let black_rating = players[j].get_rating();

                players[i].set_rating(res.0 + white_rating);
                players[j].set_rating(res.0 + black_rating);

                //println!("white score: {}, black score: {}", players[i].get_rating(), players[j].get_rating());
            }
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
                Color::Black => { (-1.5, 1.5) }
            }
        }
    };



    if out.0 > 0.0 {
        out.0 *= white_points as f64/black_points as f64;
    }
    else {
        out.0 *= black_points as f64/white_points as f64;
    }

    if out.0 > 0.0 {
        out.1 *= black_points as f64/white_points as f64;
    }
    else {
        out.1 *= white_points as f64/black_points as f64;
    }

    //(out.0 / (moves as f64 / 50.0), out.1 / (moves as f64 / 50.0))
    out
}