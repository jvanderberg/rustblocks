use std::{io::stdout, thread};
mod pieces;
use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind},
    style::{Color, Print, SetForegroundColor},
    terminal, ExecutableCommand,
};
use rand::prelude::SliceRandom;

const BLOCK: &str = "\u{2588}\u{2588}";
use pieces::Piece;
use pieces::PIECES;

#[derive(Clone)]
struct CurrentPiece {
    piece: Piece,
    x: i32,
    y: i32,
}

struct Bag {
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
#[derive(Clone)]
struct Board {
    width: usize,
    height: usize,
    cells: Vec<Vec<u8>>,
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
/// Prints the text at the given position with the given color.
///
fn print_xy(x: u16, y: u16, color: Color, text: &str, board_offset: (usize, usize)) {
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
/// Clears any lines that are full and moves the lines above down.
///
fn clear_lines(board: &mut Board) -> i32 {
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
fn draw_current_piece(current_piece: &CurrentPiece, board: &mut Board) {
    draw_piece(
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
fn draw_piece(piece: &Piece, board: &mut Board, x: i32, y: i32, color: u8) {
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
fn remove_tracer(board: &mut Board) {
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
                    "  ",
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

///
/// Calculates the new score, and as a side effect, prints the new score and level.
///
fn calc_score(lines_cleared: i32, lines: i32, score: i32) -> (i32, i32, i32) {
    let new_lines = lines + lines_cleared;
    let new_level = (new_lines / 10) + 1;

    let new_score = score
        + match lines_cleared {
            1 => 100 * new_level,
            2 => 300 * new_level,
            3 => 500 * new_level,
            4 => 800 * new_level,
            _ => 0,
        };

    print_xy(
        1,
        3,
        Color::AnsiValue(1),
        format!("Score: {}", new_score).as_str(),
        (0, 0),
    );
    print_xy(
        1,
        4,
        Color::AnsiValue(1),
        format!("Level: {}", new_level).as_str(),
        (0, 0),
    );
    print_xy(
        1,
        5,
        Color::AnsiValue(1),
        format!("Lines: {}", new_lines).as_str(),
        (0, 0),
    );
    (new_lines, new_score, new_level)
}

///
/// Print the next piece in the upper left
///
fn print_next_piece(piece: &Piece, last_piece: &Piece) {
    for square in last_piece.view() {
        print_xy(
            ((square.x + 2) * 2) as u16,
            (square.y + 2) as u16,
            Color::AnsiValue(piece.color),
            "  ",
            (1, 6),
        );
    }
    for square in piece.view() {
        print_xy(
            ((square.x + 2) * 2) as u16,
            (square.y + 2) as u16,
            Color::AnsiValue(piece.color),
            BLOCK,
            (1, 6),
        );
    }
}
fn main() -> std::io::Result<()> {
    let mut show_tracer = false;
    let mut piece_bag = Bag::new();
    let mut lines = 0;
    let mut level = 1;
    let mut score = 0;
    let width: usize = 10;
    let height: usize = 22;
    let window_size = crossterm::terminal::size()?;

    let board_offet = (
        window_size.0 as usize / 2 - width - 1,
        window_size.1 as usize / 2 - height / 2,
    );

    let initial_positon = ((width / 2) as i32, 2);
    let mut last_tick = std::time::SystemTime::now();
    let mut current_piece = CurrentPiece {
        piece: piece_bag.next(),
        x: initial_positon.0,
        y: initial_positon.1,
    };

    let mut next_piece: Piece = piece_bag.next();

    let mut next_board = Board {
        width: width + 2,
        height: height + 1,
        cells: vec![vec![0; height + 1]; width + 2],
    };
    let mut current_board = Board {
        width: width + 2,
        height: height + 1,
        cells: vec![vec![0; height + 1]; width + 2],
    };

    for i in 0..next_board.width {
        next_board.cells[i][next_board.height - 1] = 8;
    }

    for i in 0..height {
        next_board.cells[0][i] = 8;
        next_board.cells[next_board.width - 1][i] = 8;
    }
    terminal::enable_raw_mode()?;
    let _ = stdout()
        .execute(terminal::Clear(terminal::ClearType::All))?
        .execute(cursor::Hide);

    draw_diff(
        &mut current_board,
        &mut next_board,
        current_piece.piece.color,
        board_offet,
    );

    loop {
        let mut changed = false;
        // Roughly eq to 60 frames per second, though in a terminal that makes little sense as
        // keyboard repeat rate plays the biggest role in the speed of the game.
        if poll(std::time::Duration::from_millis(16))? {
            let event = read()?;
            let new_level = (lines / 10) + 1;
            if new_level != level {
                print_xy(
                    1,
                    1,
                    Color::AnsiValue(1),
                    format!("Level {}", new_level).as_str(),
                    (0, 0),
                );
                level = new_level;
            }

            changed = match event {
                Event::Key(KeyEvent {
                    kind: KeyEventKind::Press,
                    code,
                    modifiers: _,
                    state: _,
                }) => match code {
                    KeyCode::Esc => break,
                    KeyCode::Left => current_piece.move_left(&current_board),
                    KeyCode::Right => current_piece.move_right(&current_board),
                    KeyCode::Up => current_piece.rotate_right(&current_board),
                    KeyCode::Down => current_piece.move_down(&current_board),
                    KeyCode::Char('t') => {
                        show_tracer = !show_tracer;

                        true
                    }
                    KeyCode::Char(' ') => {
                        while current_piece.move_down(&current_board) {
                            thread::sleep(std::time::Duration::from_millis(10));
                            draw_current_piece(&current_piece, &mut next_board);
                            draw_diff(
                                &mut current_board,
                                &mut next_board,
                                current_piece.piece.color,
                                board_offet,
                            );
                        }
                        draw_piece(
                            &current_piece.piece,
                            &mut next_board,
                            current_piece.x,
                            current_piece.y,
                            current_piece.piece.color,
                        );

                        (lines, score, level) =
                            calc_score(clear_lines(&mut next_board), lines, score);

                        current_piece = CurrentPiece {
                            piece: next_piece,
                            x: initial_positon.0,
                            y: initial_positon.1,
                        };
                        next_piece = piece_bag.next();
                        print_next_piece(&next_piece, &current_piece.piece);
                        if current_piece.collides(&next_board, current_piece.x, current_piece.y) {
                            break;
                        }

                        true
                    }
                    KeyCode::Char('q') => break,

                    _ => false,
                },

                _ => false,
            }
        }

        let mut interval = 1000 - level * 50;
        if (interval as i32) < 250 {
            interval = 250;
        }

        // Using unwrap here is safe because we know that the system time is always valid, if it's not, we have bigger problems.
        if last_tick.elapsed().unwrap().as_millis() > interval as u128 {
            last_tick = std::time::SystemTime::now();
            let success = current_piece.move_down(&current_board);
            if !success {
                draw_piece(
                    &current_piece.piece,
                    &mut next_board,
                    current_piece.x,
                    current_piece.y,
                    current_piece.piece.color,
                );

                (lines, score, level) = calc_score(clear_lines(&mut next_board), lines, score);

                current_piece = CurrentPiece {
                    piece: next_piece,
                    x: initial_positon.0,
                    y: initial_positon.1,
                };
                next_piece = piece_bag.next();
                print_next_piece(&next_piece, &current_piece.piece);
                if current_piece.collides(&next_board, current_piece.x, current_piece.y) {
                    break;
                }
            } else {
                draw_current_piece(&current_piece, &mut next_board);
            }
            draw_diff(
                &mut current_board,
                &mut next_board,
                current_piece.piece.color,
                board_offet,
            );
        } else {
            if changed {
                draw_current_piece(&current_piece, &mut next_board);
                if show_tracer {
                    let mut tracer = current_piece.clone();
                    while tracer.move_down(&next_board) {}
                    draw_tracer(&tracer.piece, &mut next_board, tracer.x, tracer.y);
                } else {
                    remove_tracer(&mut next_board);
                }
                draw_diff(
                    &mut current_board,
                    &mut next_board,
                    current_piece.piece.color,
                    board_offet,
                );
            }
        }
    }
    terminal::disable_raw_mode()?;
    Ok(())
}
