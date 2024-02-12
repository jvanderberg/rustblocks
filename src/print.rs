use crate::{
    board::Board,
    gamestate::{EventHandler, GameEvent, GameState},
    pieces::{self, EMPTY_BLOCK},
};
use crossterm::{
    cursor,
    style::{Color, Print, SetForegroundColor},
    terminal, ExecutableCommand,
};
use pieces::{Piece, BLOCK};
use std::{cell::RefCell, io::stdout, rc::Rc};

#[derive(Clone, Default)]

struct TerminalRendererState {
    last_board: Option<Board>,
    window_size: (u16, u16),
    last_piece: Option<Piece>,
    board_offset: (u16, u16),
}

pub struct TerminalRenderer {
    state: Rc<RefCell<TerminalRendererState>>,
}

impl EventHandler for TerminalRenderer {
    fn handle_event(&mut self, gs: &GameState, ge: &GameEvent) {
        match ge {
            GameEvent::ScoreChanged | GameEvent::LinesClearedChanged | GameEvent::LevelChanged => {
                self.print_score(&gs);
            }
            GameEvent::GameStarted => {
                self.print_score(&gs);
                self.draw_next_piece(&gs.next_piece, gs.show_next_piece);
                self.draw_board(&gs.board, gs.current_piece.piece.color)
            }
            GameEvent::PieceChanged => {
                self.draw_next_piece(&gs.next_piece, gs.show_next_piece);
                self.draw_board(&gs.board, gs.current_piece.piece.color);
            }
            _ => {
                self.draw_board(&gs.board, gs.current_piece.piece.color);
            }
        }
    }
    fn clone_boxed(&self) -> Box<dyn EventHandler> {
        Box::new(Clone::clone(self))
    }
}

// impl Clone for Box<dyn EventHandler> {
//     fn clone(&self) -> Box<dyn EventHandler> {
//         self.clone_boxed()
//     }
// }
impl Clone for TerminalRenderer {
    fn clone(&self) -> TerminalRenderer {
        TerminalRenderer {
            state: Rc::clone(&self.state),
        }
    }
}
impl TerminalRenderer {
    pub fn new(window_size: (u16, u16), board_width: u16, board_height: u16) -> TerminalRenderer {
        let board_offset = get_board_offset(window_size, board_width, board_height);
        TerminalRenderer {
            state: Rc::new(RefCell::new(TerminalRendererState {
                last_board: None,
                window_size,
                last_piece: None,
                board_offset,
            })),
        }
    }

    pub fn get_window_size(&self) -> (u16, u16) {
        self.state.borrow().window_size
    }
    pub fn set_window_size(&mut self, window_size: (u16, u16)) {
        self.state.borrow_mut().window_size = window_size;
        self.state.borrow_mut().board_offset = get_board_offset(window_size, 10, 22);
    }
    pub fn draw_board(&mut self, board: &Board, current_piece_color: u8) {
        let mut state = self.state.borrow_mut();
        for y in 0..board.height {
            for x in 0..board.width {
                if let Some(prev_board) = &state.last_board {
                    if prev_board.cells[x as usize][y as usize] > 0
                        && board.cells[x as usize][y as usize] == 0
                    {
                        print_xy(
                            x as u16 * 2,
                            y as u16,
                            Color::AnsiValue(0),
                            EMPTY_BLOCK,
                            state.board_offset,
                        );
                    } else if prev_board.cells[x as usize][y as usize]
                        != board.cells[x as usize][y as usize]
                    {
                        print_xy(
                            x as u16 * 2,
                            y as u16,
                            match board.cells[x as usize][y as usize] {
                                0 => Color::AnsiValue(0),
                                254 => Color::AnsiValue(7),
                                255 => Color::AnsiValue(current_piece_color),
                                _ => Color::AnsiValue(board.cells[x as usize][y as usize]),
                            },
                            BLOCK,
                            state.board_offset,
                        );
                    }
                } else if board.cells[x as usize][y as usize] > 0 {
                    print_xy(
                        x as u16 * 2,
                        y as u16,
                        match board.cells[x as usize][y as usize] {
                            254 => Color::AnsiValue(7),
                            255 => Color::AnsiValue(current_piece_color),
                            _ => Color::AnsiValue(board.cells[x as usize][y as usize]),
                        },
                        BLOCK,
                        state.board_offset,
                    );
                }
            }
        }
        state.last_board = Some(board.clone());
    }

    pub fn refresh_board(&mut self, board: &Board, current_piece_color: u8) {
        self.clear_screen();
        {
            self.state.borrow_mut().last_board = None
        };

        self.draw_board(board, current_piece_color);
    }

    pub fn clear_screen(&self) {
        stdout()
            .execute(terminal::Clear(terminal::ClearType::All))
            .unwrap();
    }
    pub fn print_score(&self, gs: &GameState) {
        print_xy(
            3,
            1,
            Color::AnsiValue(1),
            gs.difficulty.to_string().as_str(),
            (0, 0),
        );
        print_xy(
            3 + gs.difficulty.to_string().len() as u16 + 1,
            1,
            Color::AnsiValue(1),
            "Mode",
            (0, 0),
        );
        let score_text = if gs.undo_used {
            "Score (Undo Used)"
        } else {
            "Score"
        };
        print_xy(3, 3, Color::AnsiValue(1), score_text, (0, 0));
        print_xy(
            3,
            4,
            Color::AnsiValue(1),
            format!("{}", gs.score).as_str(),
            (0, 0),
        );
        print_xy(3, 6, Color::AnsiValue(1), "Level", (0, 0));
        print_xy(
            3,
            7,
            Color::AnsiValue(1),
            format!("{}", gs.level).as_str(),
            (0, 0),
        );
        print_xy(3, 9, Color::AnsiValue(1), "Lines", (0, 0));
        print_xy(
            3,
            10,
            Color::AnsiValue(1),
            format!("{}", gs.lines).as_str(),
            (0, 0),
        );

        print_xy(3, 12, Color::AnsiValue(1), "Next Piece", (0, 0));
    }

    fn remove_next_piece(&self) {
        if let Some(last_piece) = &self.state.borrow().last_piece {
            for square in last_piece.view() {
                print_xy(
                    ((square.x + 2) * 2) as u16,
                    (square.y + 2) as u16,
                    Color::AnsiValue(1),
                    EMPTY_BLOCK,
                    (3, 13),
                );
            }
        }
    }

    ///
    /// Print the next piece in the upper left
    ///
    pub fn draw_next_piece(&mut self, piece: &Piece, show_next_piece: bool) {
        self.remove_next_piece();
        if !show_next_piece {
            return;
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
        self.state.borrow_mut().last_piece = Some(piece.clone());
    }
}
///
/// Prints the text at the given position with the given color.
///
pub fn print_xy(x: u16, y: u16, color: Color, text: &str, board_offset: (u16, u16)) {
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

pub fn show_cursor() {
    stdout().execute(cursor::Show).unwrap();
}

pub fn hide_cursor() {
    stdout().execute(cursor::Hide).unwrap();
}

static STARTUP_MESSAGE: [&str; 18] = [
    "Rustblocks is a simple tetromino based falling blocks game that uses",
    "ANSI escape sequences to draw on the terminal.",
    "",
    "Keys: Arrow keys or h,j,k,l to move",
    "      space to drop",
    "      Delete or Backspace to restart",
    "      d toggle difficulty",
    "      q to quit",
    "      u to undo",
    "      n to toggle next piece display",
    "      t key toggles the tracer block",
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

pub fn get_board_offset(
    window_size: (u16, u16),
    board_width: u16,
    board_height: u16,
) -> (u16, u16) {
    (
        (window_size.0 as usize / 2).saturating_sub(board_width as usize + 1) as u16,
        (window_size.1 as usize / 2).saturating_sub(board_height as usize / 2) as u16,
    )
}
