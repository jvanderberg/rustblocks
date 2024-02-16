use blocks_lib::pieces::{xy, Piece, PieceColor, PIECES};
use blocks_lib::{board::Board, gamestate::GameState};
use crossterm::{
    cursor,
    style::{Color, Print, SetForegroundColor},
    terminal, ExecutableCommand,
};
use std::{cell::RefCell, io::stdout};
pub const BLOCK: &str = "\u{2588}\u{2588}";
pub const EMPTY_BLOCK: &str = "  ";
#[derive(Clone, Default)]

struct TerminalRendererState {
    last_board: Option<Board>,
    window_size: (u16, u16),
    last_piece: Option<Piece>,
    board_offset: (u16, u16),
    text_color: u8,
}

#[derive(Clone)]
pub struct TerminalRenderer {
    state: RefCell<TerminalRendererState>,
}

// Map Piece colors to ANSI colors
trait PieceColorTrait {
    fn get_color(&self) -> u8;
}

impl PieceColorTrait for PieceColor {
    fn get_color(&self) -> u8 {
        match self {
            PieceColor::Wall => 8,
            PieceColor::Empty => 0,
            PieceColor::Red => 9,
            PieceColor::Green => 10,
            PieceColor::Blue => 21,
            PieceColor::Yellow => 11,
            PieceColor::Cyan => 14,
            PieceColor::Magenta => 93,
            PieceColor::Orange => 208,
            // This is never used, just a marker for the current piece.
            PieceColor::Tracer => 254,
        }
    }
}

impl TerminalRenderer {
    pub fn new(
        window_size: (u16, u16),
        board_width: u16,
        board_height: u16,
        text_color: u8,
    ) -> TerminalRenderer {
        let board_offset = get_board_offset(window_size, board_width, board_height);
        TerminalRenderer {
            state: RefCell::new(TerminalRendererState {
                last_board: None,
                window_size,
                last_piece: None,
                board_offset,
                text_color,
            }),
        }
    }

    pub fn get_window_size(&self) -> (u16, u16) {
        self.state.borrow().window_size
    }
    pub fn set_window_size(&self, window_size: (u16, u16)) {
        self.state.borrow_mut().window_size = window_size;
        self.state.borrow_mut().board_offset = get_board_offset(window_size, 10, 22);
    }
    pub fn draw_board(&self, board: &Board) {
        let mut state = self.state.borrow_mut();
        for y in 0..board.height {
            for x in 0..board.width {
                if let Some(prev_board) = &state.last_board {
                    if prev_board.cells[x as usize][y as usize] != PieceColor::Empty
                        && board.cells[x as usize][y as usize] == PieceColor::Empty
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
                            Color::AnsiValue(board.cells[x as usize][y as usize].get_color()),
                            BLOCK,
                            state.board_offset,
                        );
                    }
                } else if board.cells[x as usize][y as usize] != PieceColor::Empty {
                    print_xy(
                        x as u16 * 2,
                        y as u16,
                        Color::AnsiValue(board.cells[x as usize][y as usize].get_color()),
                        BLOCK,
                        state.board_offset,
                    );
                }
            }
        }
        state.last_board = Some(board.clone());
    }

    pub fn refresh_board(&self, gs: &GameState) {
        self.clear_screen();
        {
            self.state.borrow_mut().last_board = None
        };

        self.draw_board(&gs.get_board());
        self.draw_score(&gs);
        self.draw_next_piece(&gs.get_next_piece(), gs.get_show_next_piece());
    }

    pub fn clear_screen(&self) {
        stdout()
            .execute(terminal::Clear(terminal::ClearType::All))
            .unwrap();
    }
    pub fn draw_score(&self, gs: &GameState) {
        let (score, lines, level) = gs.get_score();
        print_xy(
            3,
            1,
            Color::AnsiValue(self.state.borrow().text_color),
            gs.get_difficulty().to_string().as_str(),
            (0, 0),
        );
        print_xy(
            3 + gs.get_difficulty().to_string().len() as u16 + 1,
            1,
            Color::AnsiValue(self.state.borrow().text_color),
            "Mode",
            (0, 0),
        );
        let score_text = if gs.get_undo_used() {
            "Score (Undo Used)"
        } else {
            "Score"
        };
        print_xy(
            3,
            3,
            Color::AnsiValue(self.state.borrow().text_color),
            score_text,
            (0, 0),
        );
        print_xy(
            3,
            4,
            Color::AnsiValue(self.state.borrow().text_color),
            format!("{}", score).as_str(),
            (0, 0),
        );
        print_xy(
            3,
            6,
            Color::AnsiValue(self.state.borrow().text_color),
            "Level",
            (0, 0),
        );
        print_xy(
            3,
            7,
            Color::AnsiValue(self.state.borrow().text_color),
            format!("{}", level).as_str(),
            (0, 0),
        );
        print_xy(
            3,
            9,
            Color::AnsiValue(self.state.borrow().text_color),
            "Lines",
            (0, 0),
        );
        print_xy(
            3,
            10,
            Color::AnsiValue(self.state.borrow().text_color),
            format!("{}", lines).as_str(),
            (0, 0),
        );

        print_xy(
            3,
            12,
            Color::AnsiValue(self.state.borrow().text_color),
            "Next Piece",
            (0, 0),
        );
    }

    fn remove_next_piece(&self) {
        if let Some(last_piece) = &self.state.borrow().last_piece {
            for square in last_piece.view() {
                print_xy(
                    ((xy(&square).0 + 2) * 2) as u16,
                    (xy(&square).1 + 2) as u16,
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
    pub fn draw_next_piece(&self, piece: &Piece, show_next_piece: bool) {
        self.remove_next_piece();
        if !show_next_piece {
            return;
        }
        for square in piece.view() {
            print_xy(
                ((xy(&square).0 + 2) * 2) as u16,
                (xy(&square).1 + 2) as u16,
                Color::AnsiValue(piece.color.get_color()),
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
    for piece in PIECES.iter() {
        for square in piece.view() {
            print_xy(
                ((xy(&square).0 * 2) + 2) as u16,
                (xy(&square).1 + 2) as u16,
                Color::AnsiValue(piece.color.get_color()),
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
