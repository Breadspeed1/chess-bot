use crate::game::Piece::{BISHOP, EMPTY, KING, KNIGHT, PAWN, QUEEN, ROOK};
use crate::game::Team::BLACK;
use crate::game::Team::WHITE;

enum Team {
    WHITE,
    BLACK
}

enum Piece {
    EMPTY,
    PAWN(Team),
    KNIGHT(Team),
    ROOK(Team, bool),
    BISHOP(Team),
    QUEEN(Team),
    KING(Team, bool),
}

struct Board {
    data: [Piece; 64]
}

impl Board {
    fn empty() -> Board {
        Board {
            //cursed lmao
            data: [
                ROOK(BLACK, true), KNIGHT(BLACK), BISHOP(BLACK), QUEEN(BLACK), KING(BLACK, true), BISHOP(BLACK), KNIGHT(BLACK), ROOK(BLACK, true),
                PAWN(BLACK), PAWN(BLACK), PAWN(BLACK), PAWN(BLACK), PAWN(BLACK), PAWN(BLACK), PAWN(BLACK), PAWN(BLACK),
                EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
                EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
                EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
                EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
                ROOK(WHITE, true), KNIGHT(WHITE), BISHOP(WHITE), QUEEN(WHITE), KING(WHITE, true), BISHOP(WHITE), KNIGHT(WHITE), ROOK(WHITE, true),
                PAWN(WHITE), PAWN(WHITE), PAWN(WHITE), PAWN(WHITE), PAWN(WHITE), PAWN(WHITE), PAWN(WHITE), PAWN(WHITE),
            ]
        }
    }

    fn move_piece(&self, x1: u8, y1: u8, x2: u8, y2: u8) -> bool {
        todo!();

        if self.is_valid_move(x1, y1, x2, y2) {

            return true;
        }

        false
    }

    fn is_valid_move(&self, x1: u8, y1: u8, x2: u8, y2: u8) -> bool {
        let start: &Piece = &self.data[y1 as usize * 8 + x1 as usize];
        let end: &Piece = &self.data[y2 as usize * 8 + x2 as usize];

        todo!()
    }
}