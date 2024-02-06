use crossterm::style::Color;
use rand::seq::SliceRandom;

use crate::{
    pieces::{Piece, BLOCK, EMPTY_BLOCK, PIECES},
    print::print_xy,
};

#[derive(Clone)]
pub struct Board {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<u8>>,
}

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

    pub fn peek(&mut self) -> &Piece {
        if self.pieces.len() == 0 {
            self.pieces = PIECES.to_vec();
            let mut rng = rand::thread_rng();
            self.pieces.shuffle(&mut rng);
        }
        &self.pieces[self.pieces.len() - 1]
    }
}

#[derive(Clone)]
pub struct CurrentPiece {
    pub piece: Piece,
    pub x: i32,
    pub y: i32,
}

///
/// Clears any lines that are full and moves the lines above down.
///
pub fn clear_lines(board: &mut Board) -> i32 {
    let mut y = board.height - 2;
    let mut lines = 0;
    while y > 0 {
        if (0..board.width).all(|x| board.cells[x][y] > 0) {
            lines += 1;
            for y2 in (1..=y).rev() {
                for x in 0..board.width {
                    board.cells[x][y2] = board.cells[x][y2 - 1];
                }
            }
        } else {
            y -= 1;
        }
    }
    lines
}

///
/// Draws the current piece on the board.
///
pub fn commit_current_piece(current_piece: &CurrentPiece, board: &mut Board) {
    commit_piece(
        &current_piece.piece,
        board,
        current_piece.x,
        current_piece.y,
        255,
    );
}

///
/// Draws the piece on the board.
///
pub fn commit_piece(piece: &Piece, board: &mut Board, x: i32, y: i32, color: u8) {
    // Clear out the current position of the piece, if any.
    for y in 0..board.height {
        for x in 0..board.width {
            if board.cells[x][y] >= 255 {
                board.cells[x][y] = 0;
            }
        }
    }
    for square in piece.view() {
        let x = square.x + x as i32;
        let y = square.y + y as i32;
        board.cells[x as usize][y as usize] = color;
    }
}

///
/// Removes the tracer from the board.
///
pub fn remove_tracer(board: &mut Board) {
    // Clear out the current position of the piece, if any.
    for y in 0..board.height {
        for x in 0..board.width {
            if board.cells[x][y] == 254 {
                board.cells[x][y] = 0;
            }
        }
    }
}
///
/// Draws the piece on the board.
///
fn draw_tracer(piece: &Piece, board: &mut Board, x: i32, y: i32) {
    // Clear out the current position of the piece, if any.
    remove_tracer(board);
    for square in piece.view() {
        let x = square.x + x as i32;
        let y = square.y + y as i32;
        board.cells[x as usize][y as usize] = 254;
    }
}
///
/// Compares the current board with the next board and draws the differences.
/// Copies changes from the next board to the current board, and then swaps the two boards.
///
fn draw_diff<'a>(
    current_board: &mut Board,
    next_board: &mut Board,
    current_piece_color: u8,
    board_offset: (usize, usize),
) {
    for y in 0..next_board.height {
        for x in 0..next_board.width {
            if (current_board.cells[x][y] > 0) && (next_board.cells[x][y] == 0) {
                print_xy(
                    x as u16 * 2,
                    y as u16,
                    Color::AnsiValue(0),
                    EMPTY_BLOCK,
                    board_offset,
                );
                current_board.cells[x][y] = 0;
            } else if current_board.cells[x][y] != next_board.cells[x][y] {
                print_xy(
                    x as u16 * 2,
                    y as u16,
                    match next_board.cells[x][y] {
                        0 => Color::AnsiValue(0),
                        254 => Color::AnsiValue(7),
                        255 => Color::AnsiValue(current_piece_color),
                        _ => Color::AnsiValue(next_board.cells[x][y]),
                    },
                    BLOCK,
                    board_offset,
                );
                current_board.cells[x][y] = next_board.cells[x][y];
            }
        }
    }
    std::mem::swap(next_board, current_board);
}

pub fn update_board(
    current_piece: &CurrentPiece,
    current_board: &mut Board,
    next_board: &mut Board,
    show_tracer: bool,
    board_offet: (usize, usize),
) {
    commit_current_piece(&current_piece, next_board);
    if show_tracer {
        let mut tracer = current_piece.clone();
        while tracer.move_down(next_board) {}
        draw_tracer(&tracer.piece, next_board, tracer.x, tracer.y);
    } else {
        remove_tracer(next_board);
    }
    draw_diff(
        current_board,
        next_board,
        current_piece.piece.color,
        board_offet,
    );
}

impl CurrentPiece {
    pub fn collides(&self, board: &Board, x: i32, y: i32) -> bool {
        for square in self.piece.view() {
            let x = square.x + x as i32;
            let y = square.y + y as i32;
            if x < 0 || x >= board.width as i32 || y >= board.height as i32 {
                return true;
            }
            if y >= 0
                && board.cells[x as usize][y as usize] > 0
                && board.cells[x as usize][y as usize] < 254
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
