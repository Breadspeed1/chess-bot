use owlchess::{Board, Color, Coord, Piece};

pub fn get_points(p: Piece) -> usize {
    match p {
        Piece::Pawn => { 1 }
        Piece::King => { 0 }
        Piece::Knight => { 3 }
        Piece::Bishop => { 3 }
        Piece::Rook => { 5 }
        Piece::Queen => { 9 }
    }
}

pub fn get_id(p: Piece) -> usize {
    match p {
        Piece::Pawn => { 0 }
        Piece::King => { 1 }
        Piece::Knight => { 2 }
        Piece::Bishop => { 3 }
        Piece::Rook => { 4 }
        Piece::Queen => { 5 }
    }
}

pub fn get_color_points(board: &Board, color: &Color) -> usize {
    let mut total: usize = 0;

    for x in 0..63 as usize {
        let cell = board.get(Coord::from_index(x));
        if let Some(piece) = cell.piece() {
            if *color == cell.color().unwrap() {
                total += get_points(piece);
            }
        }
    }

    total
}