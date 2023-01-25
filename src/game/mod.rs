use crate::game::Piece::{BISHOP, EMPTY, KING, KNIGHT, PAWN, QUEEN, ROOK};
use crate::game::Team::BLACK;
use crate::game::Team::WHITE;

#[derive(Clone)]
enum Team {
    WHITE,
    BLACK
}

#[derive(Clone)]
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

    fn move_piece(&mut self, x1: u8, y1: u8, x2: u8, y2: u8) -> bool {
        if self.is_valid_move(x1, y1, x2, y2) {
            //TODO: how to unretart idk
            let p = self.get(x1, y1);
            self.set(x2, y2, p);
            return true;
        }

        false
    }

    fn get(&self, x: u8, y: u8) -> &Piece {
        &self.data[y as usize * 8 + x as usize]
    }

    fn set(&mut self, x: u8, y: u8, p: &Piece) {
        self.data[y as usize * 8 + x as usize] = p.clone();
    }

    fn is_valid_move(&self, x1: u8, y1: u8, x2: u8, y2: u8) -> bool {
        let start: &Piece = self.get(x1, y1);
        let end: &Piece = self.get(x2, y2);

        todo!()
    }
}