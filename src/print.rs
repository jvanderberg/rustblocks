use blocks_lib::gamestate::Emoji;
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
    emoji: Emoji,
}

#[derive(Clone)]
pub struct TerminalRenderer {
    state: RefCell<TerminalRendererState>,
}

// Map Piece colors to ANSI colors
trait PieceColorTrait {
    fn get_color(&self, emoji: Emoji) -> (String, u8);
}

impl PieceColorTrait for PieceColor {
    fn get_color(&self, emoji: Emoji) -> (String, u8) {
        match emoji {
            Emoji::Block => match self {
                PieceColor::Wall => ('⬜'.to_string(), 8),
                PieceColor::Empty => ('⬛'.to_string(), 0),
                PieceColor::Red => ('🟥'.to_string(), 9),
                PieceColor::Green => ('🟩'.to_string(), 10),
                PieceColor::Blue => ('🟦'.to_string(), 21),
                PieceColor::Yellow => ('🟨'.to_string(), 11),
                PieceColor::Cyan => ('🟫'.to_string(), 14),
                PieceColor::Magenta => ('🟪'.to_string(), 93),
                PieceColor::Orange => ('🟧'.to_string(), 208),
                PieceColor::Tracer => ('⬜'.to_string(), 254),
            },
            Emoji::Circle => match self {
                PieceColor::Wall => ('⚪'.to_string(), 8),
                PieceColor::Empty => ('⚫'.to_string(), 0),
                PieceColor::Red => ('🔴'.to_string(), 9),
                PieceColor::Green => ('🟢'.to_string(), 10),
                PieceColor::Blue => ('🔵'.to_string(), 21),
                PieceColor::Yellow => ('🟡'.to_string(), 11),
                PieceColor::Cyan => ('🟤'.to_string(), 14),
                PieceColor::Magenta => ('🟣'.to_string(), 93),
                PieceColor::Orange => ('🟠'.to_string(), 208),
                PieceColor::Tracer => ('⚪'.to_string(), 254),
            },
            Emoji::Heart => match self {
                PieceColor::Wall => ('🤍'.to_string(), 8),
                PieceColor::Empty => ('🖤'.to_string(), 0),
                PieceColor::Red => ('🩵'.to_string(), 9),
                PieceColor::Green => ('💚'.to_string(), 10),
                PieceColor::Blue => ('💙'.to_string(), 21),
                PieceColor::Yellow => ('💛'.to_string(), 11),
                PieceColor::Cyan => ('🤎'.to_string(), 14),
                PieceColor::Magenta => ('💜'.to_string(), 93),
                PieceColor::Orange => ('🧡'.to_string(), 208),
                PieceColor::Tracer => ('🤍'.to_string(), 254),
            },

            Emoji::None => match self {
                PieceColor::Wall => (BLOCK.to_string(), 8),
                PieceColor::Empty => (BLOCK.to_string(), 0),
                PieceColor::Red => (BLOCK.to_string(), 9),
                PieceColor::Green => (BLOCK.to_string(), 10),
                PieceColor::Blue => (BLOCK.to_string(), 21),
                PieceColor::Yellow => (BLOCK.to_string(), 11),
                PieceColor::Cyan => (BLOCK.to_string(), 14),
                PieceColor::Magenta => (BLOCK.to_string(), 93),
                PieceColor::Orange => (BLOCK.to_string(), 208),
                // This is never used, just a marker for the current piece.
                PieceColor::Tracer => (BLOCK.to_string(), 254),
            },
        }
    }
}

impl TerminalRenderer {
    pub fn new(
        window_size: (u16, u16),
        board_width: u16,
        board_height: u16,
        text_color: u8,
        emoji: Emoji,
    ) -> TerminalRenderer {
        let board_offset = get_board_offset(window_size, board_width, board_height);
        TerminalRenderer {
            state: RefCell::new(TerminalRendererState {
                last_board: None,
                window_size,
                last_piece: None,
                board_offset,
                text_color,
                emoji,
            }),
        }
    }

    pub fn cycle_emoji(&self, gs: &GameState) {
        {
            let mut state = self.state.borrow_mut();
            state.emoji = match state.emoji {
                Emoji::Block => Emoji::Circle,
                Emoji::Circle => Emoji::Heart,
                Emoji::Heart => Emoji::None,
                Emoji::None => Emoji::Block,
            };
        }

        self.refresh_board(gs);
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
                let (text, color) = board.cells[x as usize][y as usize].get_color(state.emoji);

                if let Some(prev_board) = &state.last_board {
                    if prev_board.cells[x as usize][y as usize] != PieceColor::Empty
                        && board.cells[x as usize][y as usize] == PieceColor::Empty
                    {
                        print_xy(
                            x as u16 * 2,
                            y as u16,
                            Color::AnsiValue(color),
                            EMPTY_BLOCK.to_string(),
                            state.board_offset,
                        );
                    } else if prev_board.cells[x as usize][y as usize]
                        != board.cells[x as usize][y as usize]
                    {
                        print_xy(
                            x as u16 * 2,
                            y as u16,
                            Color::AnsiValue(color),
                            text,
                            state.board_offset,
                        );
                    }
                } else if board.cells[x as usize][y as usize] != PieceColor::Empty {
                    print_xy(
                        x as u16 * 2,
                        y as u16,
                        Color::AnsiValue(color),
                        text,
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
            gs.get_difficulty().to_string(),
            (0, 0),
        );
        print_xy(
            3 + gs.get_difficulty().to_string().len() as u16 + 1,
            1,
            Color::AnsiValue(self.state.borrow().text_color),
            "Mode".to_string(),
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
            score_text.to_string(),
            (0, 0),
        );
        print_xy(
            3,
            4,
            Color::AnsiValue(self.state.borrow().text_color),
            format!("{}", score),
            (0, 0),
        );
        print_xy(
            3,
            6,
            Color::AnsiValue(self.state.borrow().text_color),
            "Level".to_string(),
            (0, 0),
        );
        print_xy(
            3,
            7,
            Color::AnsiValue(self.state.borrow().text_color),
            format!("{}", level),
            (0, 0),
        );
        print_xy(
            3,
            9,
            Color::AnsiValue(self.state.borrow().text_color),
            "Lines".to_string(),
            (0, 0),
        );
        print_xy(
            3,
            10,
            Color::AnsiValue(self.state.borrow().text_color),
            format!("{}", lines),
            (0, 0),
        );

        print_xy(
            3,
            12,
            Color::AnsiValue(self.state.borrow().text_color),
            "Next Piece".to_string(),
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
                    EMPTY_BLOCK.to_string(),
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
            let (text, color) = piece.color.get_color(self.state.borrow().emoji);

            print_xy(
                ((xy(&square).0 + 2) * 2) as u16,
                (xy(&square).1 + 2) as u16,
                Color::AnsiValue(color),
                text,
                (3, 13),
            );
        }
        self.state.borrow_mut().last_piece = Some(piece.clone());
    }
}
///
/// Prints the text at the given position with the given color.
///
pub fn print_xy(x: u16, y: u16, color: Color, text: String, board_offset: (u16, u16)) {
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
    "      b to toggle block emojies",
    "      q to quit",
    "      u to undo",
    "      n to toggle next piece display",
    "      t key toggles the tracer block",
    "",
    "",
    "",
    "",
    "",
    "Press any key to continue",
];

pub fn print_startup(color: u8, emoji: Emoji) {
    for (i, line) in STARTUP_MESSAGE.iter().enumerate() {
        print_xy(
            5,
            5 + i as u16,
            Color::AnsiValue(color),
            line.to_string(),
            (0, 0),
        );
    }
    let mut x = 3;
    let y = 1;
    for piece in PIECES.iter() {
        for square in piece.view() {
            let (text, color) = piece.color.get_color(emoji);

            print_xy(
                ((xy(&square).0 * 2) + 2) as u16,
                (xy(&square).1 + 2) as u16,
                Color::AnsiValue(color),
                text,
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
