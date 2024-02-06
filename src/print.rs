use crate::pieces::{self, EMPTY_BLOCK};
use crossterm::{
    cursor,
    style::{Color, Print, SetForegroundColor},
    ExecutableCommand,
};
use pieces::{Piece, BLOCK};
use std::io::stdout;

///
/// Prints the text at the given position with the given color.
///
pub fn print_xy(x: u16, y: u16, color: Color, text: &str, board_offset: (usize, usize)) {
    // Here we use unwrap, because really we want to crash if we can't display anything
    let _ = stdout()
        .execute(cursor::MoveTo(
            x + board_offset.0 as u16,
            y + board_offset.1 as u16,
        ))
        .unwrap()
        .execute(SetForegroundColor(color))
        .unwrap()
        .execute(Print(text));
}

///
/// Print the next piece in the upper left
///
pub fn print_next_piece(piece: &Piece, last_piece: &Piece) {
    for square in last_piece.view() {
        print_xy(
            ((square.x + 2) * 2) as u16,
            (square.y + 2) as u16,
            Color::AnsiValue(piece.color),
            EMPTY_BLOCK,
            (3, 13),
        );
    }
    for square in piece.view() {
        print_xy(
            ((square.x + 2) * 2) as u16,
            (square.y + 2) as u16,
            Color::AnsiValue(piece.color),
            BLOCK,
            (3, 13),
        );
    }
}

static STARTUP_MESSAGE: [&str; 18] = [
    "Rustblocks is a simple tetromino based falling blocks game that uses",
    "ANSI escape sequences to draw on the terminal.",
    "",
    "Keys: Arrow keys or h,j,k,l to move",
    "      space to drop",
    "      q to quit",
    "      t key toggles the tracer block",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "Press any key to continue",
];

pub fn print_startup(color: u8) {
    for (i, line) in STARTUP_MESSAGE.iter().enumerate() {
        print_xy(5, 5 + i as u16, Color::AnsiValue(color), line, (0, 0));
    }
    let mut x = 3;
    let y = 1;
    for piece in pieces::PIECES.iter() {
        for square in piece.view() {
            print_xy(
                ((square.x * 2) + 2) as u16,
                ((square.y) + 2) as u16,
                Color::AnsiValue(piece.color),
                BLOCK,
                (x, y),
            );
        }
        x += 10;
    }
}
