///
/// The board module contains the logic for the game board and the pieces.
/// The board is represented as a 2D array of u8, where 0 is an empty cell and any other value is a piece.
/// 255 is a special value used to draw the current piece. 254 is the tracer piece.
///
use rand::seq::SliceRandom;

use crate::pieces::{xy, Piece, PieceColor, PIECES};

#[derive(Clone)]
pub struct Board {
    pub width: u16,
    pub height: u16,
    pub cells: Vec<Vec<PieceColor>>,
}

#[derive(Clone)]
pub struct Bag {
    pieces: Vec<Piece>,
}

impl Bag {
    pub fn new() -> Bag {
        let mut pieces = PIECES.to_vec();
        let mut rng = rand::thread_rng();
        pieces.shuffle(&mut rng);
        Bag { pieces }
    }

    pub fn next(&mut self) -> Piece {
        if self.pieces.len() == 0 {
            self.pieces = PIECES.to_vec();
            let mut rng = rand::thread_rng();
            self.pieces.shuffle(&mut rng);
        }
        // This is safe because we just checked the length.
        self.pieces.pop().unwrap()
    }
}

#[derive(Clone, Debug)]
pub struct CurrentPiece {
    pub piece: Piece,
    pub x: i32,
    pub y: i32,
}

///
/// Clears any lines that are full and moves the lines above down.
///
pub fn clear_lines(board: &mut Board) -> i32 {
    let mut y = board.height as usize - 2;
    let mut lines = 0;
    while y > 0 {
        if (0..board.width).all(|x| board.cells[x as usize][y] != PieceColor::Empty) {
            lines += 1;
            for y2 in (1..=y).rev() {
                for x in 0..board.width {
                    board.cells[x as usize][y2] = board.cells[x as usize][y2 - 1];
                }
            }
        } else {
            y -= 1;
        }
    }
    lines
}

///
/// Draws the piece on the board.
///
pub fn draw_piece(piece: &Piece, board: &mut Board, x: i32, y: i32, color: PieceColor) {
    for square in piece.view() {
        let x = xy(&square).0 as i32 + x as i32;
        let y = xy(&square).1 as i32 + y as i32;
        board.cells[x as usize][y as usize] = color;
    }
}

pub fn remove_piece(piece: &Piece, board: &mut Board, x: i32, y: i32) {
    for square in piece.view() {
        let x = xy(&square).0 as i32 + x as i32;
        let y = xy(&square).1 as i32 + y as i32;
        board.cells[x as usize][y as usize] = PieceColor::Empty;
    }
}

///
/// Removes the tracer from the board.
///
pub fn remove_tracer(board: &mut Board) {
    // Clear out the current position of the piece, if any.
    for y in 0..board.height {
        for x in 0..board.width {
            if board.cells[x as usize][y as usize] == PieceColor::Tracer {
                board.cells[x as usize][y as usize] = PieceColor::Empty;
            }
        }
    }
}
///
/// Draws the tracer piece on the board.
///
pub fn draw_tracer(piece: &Piece, board: &mut Board, x: i32, y: i32) {
    // Clear out the current position of the piece, if any.
    remove_tracer(board);
    for square in piece.view() {
        let x = xy(&square).0 as i32 + x as i32;
        let y = xy(&square).1 as i32 + y as i32;
        board.cells[x as usize][y as usize] = PieceColor::Tracer;
    }
}

impl CurrentPiece {
    pub fn collides(&self, board: &Board, x: i32, y: i32) -> bool {
        for square in self.piece.view() {
            let dx = xy(&square).0 as i32;
            let dy = xy(&square).1 as i32;
            let x = dx as i32 + x as i32;
            let y = dy as i32 + y as i32;

            if x < 0 || x >= board.width as i32 || y >= board.height as i32 {
                return true;
            }
            if y >= 0
                && board.cells[x as usize][y as usize] != PieceColor::Empty
                && board.cells[x as usize][y as usize] != PieceColor::Tracer
            {
                return true;
            }
        }
        false
    }

    pub fn rotate_right(&mut self, board: &Board) -> bool {
        self.piece.rotate_right();
        if self.collides(board, self.x, self.y) {
            self.piece.rotate_left();
            return false;
        }
        true
    }
    pub fn move_left(&mut self, board: &Board) -> bool {
        let x = self.x - 1;
        if !self.collides(board, x, self.y) {
            self.x = x;
            return true;
        }
        false
    }
    pub fn move_right(&mut self, board: &Board) -> bool {
        let x = self.x + 1;
        if !self.collides(board, x, self.y) {
            self.x = x;
            return true;
        }
        false
    }

    pub fn move_down(&mut self, board: &Board) -> bool {
        let y = self.y + 1;
        if !self.collides(board, self.x, y) {
            self.y = y;
            return true;
        }
        false
    }
}
