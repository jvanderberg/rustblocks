use rand::seq::SliceRandom;

use crate::{
    gamestate::{Difficulty, GameState},
    pieces::{Piece, PIECES},
};

#[derive(Clone)]
pub struct Board {
    pub width: u16,
    pub height: u16,
    pub cells: Vec<Vec<u8>>,
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
pub fn clear_lines(gs: &mut GameState) -> i32 {
    let mut y = gs.board.height as usize - 2;
    let mut lines = 0;
    while y > 0 {
        if (0..gs.board.width).all(|x| gs.board.cells[x as usize][y] > 0) {
            lines += 1;
            for y2 in (1..=y).rev() {
                for x in 0..gs.board.width {
                    gs.board.cells[x as usize][y2] = gs.board.cells[x as usize][y2 - 1];
                }
            }
        } else {
            y -= 1;
        }
    }
    lines
}

///
/// Draws the current piece on the board, using the 'special' color 255.
/// The 'draw_diff' function will then draw the piece in the correct color using the color
/// the gs.current_piece
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
            if board.cells[x as usize][y as usize] >= 255 {
                board.cells[x as usize][y as usize] = 0;
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
            if board.cells[x as usize][y as usize] == 254 {
                board.cells[x as usize][y as usize] = 0;
            }
        }
    }
}
///
/// Draws the tracer piece on the board.
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
/// Updates the board after a change
///
pub fn update_board(gs: &mut crate::gamestate::GameState) {
    commit_current_piece(&gs.current_piece, &mut gs.board);

    if gs.show_tracer {
        let mut tracer = gs.current_piece.clone();
        while tracer.move_down(&gs.board) {}
        draw_tracer(&tracer.piece, &mut gs.board, tracer.x, tracer.y);
    } else {
        remove_tracer(&mut gs.board);
    }
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

///
/// Handles the piece hitting the bottom of the board.
/// This includes committing the piece to the board, clearing any lines, and updating the score.
/// It also sets the next piece to the current piece and gets a new next piece from the piece bag, and
/// prints the new next piece on the board.
///

pub fn piece_hit_bottom(gs: &mut GameState) -> i32 {
    // Commit the piece to the board with it's actual color, not the '255' current piece color.
    // This makes it 'permanent' on the board.
    commit_piece(
        &gs.current_piece.piece,
        &mut gs.board,
        gs.current_piece.x,
        gs.current_piece.y,
        gs.current_piece.piece.color,
    );
    gs.pieces += 1;
    let lines = clear_lines(gs);

    gs.current_piece = CurrentPiece {
        piece: gs.next_piece.clone(),
        x: gs.get_initial_position().0 as i32,
        y: gs.get_initial_position().1 as i32,
    };
    gs.next_piece = gs.piece_bag.next();
    return lines;
}

// pub fn refresh_board(gs: &mut GameState) {
//     // clear_board();

//     // gs.current_board = Board {
//     //     width: gs.width + 2,
//     //     height: gs.height + 1,
//     //     cells: vec![vec![0; gs.height as usize + 1]; gs.width as usize + 2],
//     // };
//     update_board(gs);
//     // update_score(gs, 0);
// }

pub fn initialize_board_pieces(gs: &mut GameState) {
    // Based on the difficulty, we want to introduce some random pieces, move them randomly, and drop them
    // to make the game more interesting.

    let extra_pieces = match gs.difficulty {
        Difficulty::Easy | Difficulty::Medium => 0,
        Difficulty::Hard => 5,
        Difficulty::Insane => 10,
    };
    for _i in 0..extra_pieces {
        let mut piece: CurrentPiece = CurrentPiece {
            piece: gs.piece_bag.next().clone(),
            x: gs.get_initial_position().0 as i32,
            y: gs.get_initial_position().1 as i32,
        };

        piece.rotate_right(&gs.board);
        // Randomly move the piece left or right
        let int = rand::random::<i32>() % (gs.width - gs.width / 2) as i32;
        if int > 0 {
            for _ in 0..int {
                piece.move_left(&gs.board);
            }
        } else {
            for _ in 0..int.abs() {
                piece.move_right(&gs.board);
            }
        }

        while piece.move_down(&gs.board) {}
        commit_piece(
            &piece.piece,
            &mut gs.board,
            piece.x,
            piece.y,
            piece.piece.color,
        );
        update_board(gs);
    }

    for i in 0..gs.board.width {
        gs.board.cells[i as usize][gs.board.height.saturating_sub(1) as usize] = 8;
    }

    for i in 0..gs.height {
        gs.board.cells[0][i as usize] = 8;
        gs.board.cells[(gs.board.width.saturating_sub(1)) as usize][i as usize] = 8;
    }
}
