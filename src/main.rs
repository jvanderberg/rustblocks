use std::io::stdout;
mod pieces;
use rand::Rng;

use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind},
    style::{Color, Print, SetForegroundColor},
    terminal, ExecutableCommand,
};
const BLOCK: &str = "\u{2588}\u{2588}";
use pieces::Piece;
use pieces::PIECES;

#[derive(Clone)]
struct CurrentPiece {
    piece: Piece,
    x: u16,
    y: u16,
}
#[derive(Clone)]
struct Board {
    width: usize,
    height: usize,
    cells: Vec<Vec<u8>>,
}

impl CurrentPiece {
    fn collides(&self, board: &Board, x: u16, y: u16) -> bool {
        for square in self.piece.view() {
            let x = square.x + x as i32;
            let y = square.y + y as i32;
            if x < 0 || x >= board.width as i32 || y >= board.height as i32 {
                return true;
            }
            if y >= 0
                && board.cells[x as usize][y as usize] > 0
                && board.cells[x as usize][y as usize] < 255
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
fn print_xy(x: u16, y: u16, color: Color, text: &str, board_offset: (usize, usize)) {
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

fn print_piece(piece: &Piece, x: u16, y: u16, board_offset: (usize, usize)) {
    for square in piece.view() {
        print_xy(
            (2 * (square.x + x as i32)) as u16,
            (square.y + y as i32) as u16,
            Color::AnsiValue(piece.color),
            BLOCK,
            board_offset,
        );
    }
}
fn print_board(board: &Board, board_offset: (usize, usize)) {
    let _ = stdout().execute(terminal::Clear(terminal::ClearType::All));

    for y in 0..board.height {
        for x in 0..board.width {
            if board.cells[x][y] > 0 && board.cells[x][y] < 255 {
                print_xy(
                    x as u16 * 2,
                    y as u16,
                    Color::AnsiValue(board.cells[x][y]),
                    BLOCK,
                    board_offset,
                );
            }
        }
        println!();
    }
}

fn draw_current_piece(current_piece: &CurrentPiece, board: &mut Board) {
    draw_piece(
        &current_piece.piece,
        board,
        current_piece.x,
        current_piece.y,
        255,
    );
}

fn clear_lines(board: &mut Board) {
    let mut y = board.height - 2;
    while y > 0 {
        if (0..board.width).all(|x| board.cells[x][y] > 0) {
            for y2 in (1..=y).rev() {
                for x in 0..board.width {
                    board.cells[x][y2] = board.cells[x][y2 - 1];
                }
            }
        } else {
            y -= 1;
        }
    }
}

fn draw_piece(piece: &Piece, board: &mut Board, x: u16, y: u16, color: u8) {
    // Clear out the current position of the piece, if any.
    for y in 0..board.height {
        for x in 0..board.width {
            if board.cells[x][y] == 255 {
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
fn draw_diff<'a>(
    current_board: &mut Board,
    next_board: &mut Board,
    current_piece_color: u8,
    board_offset: (usize, usize),
) {
    // let mut new_board = board.clone();
    // draw_piece(
    //     &current_piece.piece,
    //     &mut new_board,
    //     current_piece.x as u16,
    //     current_piece.y as u16,
    //     255,
    // );
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
fn main() -> std::io::Result<()> {
    let width: usize = 10;
    let height: usize = 22;
    let window_size = crossterm::terminal::size()?;

    if ((window_size.0 as usize) < width + 2) || ((window_size.1 as usize) < height + 2) {
        println!("Please resize the window to at least 40x25");
        return Ok(());
    }
    let board_offet = (
        window_size.0 as usize / 2 - width - 1,
        window_size.1 as usize / 2 - height / 2,
    );

    let initial_positon = (width / 2, 2);
    let mut rng = rand::thread_rng();
    let mut last_tick = std::time::SystemTime::now();
    let mut current_piece = CurrentPiece {
        piece: PIECES[1].clone(),
        x: initial_positon.0 as u16,
        y: initial_positon.1 as u16,
    };

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
        current_board.cells[i][current_board.height - 1] = 8;
    }

    for i in 0..height {
        next_board.cells[0][i] = 8;
        next_board.cells[next_board.width - 1][i] = 8;
        current_board.cells[0][i] = 8;
        current_board.cells[current_board.width - 1][i] = 8;
    }
    terminal::enable_raw_mode()?;
    let _ = stdout()
        .execute(terminal::Clear(terminal::ClearType::All))?
        .execute(cursor::Hide);

    print_board(&current_board, board_offet);

    loop {
        let mut changed = false;
        if poll(std::time::Duration::from_millis(10))? {
            let event = read()?;
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
                    KeyCode::Char(' ') => {
                        while current_piece.move_down(&current_board) {
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

                        clear_lines(&mut next_board);
                        current_piece = CurrentPiece {
                            piece: PIECES[rng.gen_range(0..6)].clone(),
                            x: initial_positon.0 as u16,
                            y: initial_positon.1 as u16,
                        };

                        true
                    }
                    KeyCode::Char('q') => break,
                    KeyCode::Char('c') => {
                        print_board(&current_board, board_offet);
                        true
                    }
                    _ => false,
                },

                _ => false,
            }
        }
        if last_tick.elapsed().unwrap().as_millis() > 1000 {
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

                clear_lines(&mut next_board);
                current_piece = CurrentPiece {
                    piece: PIECES[rng.gen_range(0..6)].clone(),
                    x: initial_positon.0 as u16,
                    y: initial_positon.1 as u16,
                };
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
