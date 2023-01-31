use owlchess::Piece;

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