use crate::{
    board::{Bag, Board, CurrentPiece},
    pieces::Piece,
};
use clap::{arg, command, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None,)]
pub struct Args {
    /// The width of the board
    #[arg(short = 'x', long, default_value = "10")]
    horizontal: u16,
    /// The height of the board
    #[arg(short = 'y', long, default_value = "22")]
    vertical: u16,

    /// Whether to show the next piece
    #[arg(short = 'n', long, default_value = "false")]
    hide_next_piece: bool,
}

#[derive(Clone)]
pub struct GameState {
    pub current_piece: CurrentPiece,
    pub next_piece: Piece,
    pub next_board: Board,
    pub current_board: Board,
    pub piece_bag: Bag,
    pub lines: i32,
    pub level: i32,
    pub score: i32,
    pub show_tracer: bool,
    pub show_next_piece: bool,
    pub startup_screen: bool,
    pub window_size: (u16, u16),
    pub board_offset: (u16, u16),
    pub initial_positon: (u16, u16),
    pub width: u16,
    pub height: u16,
    pub undo_used: bool,
    pub pieces: u16,
}

fn get_board_offset(width: u16, height: u16, window_size: (u16, u16)) -> (u16, u16) {
    (
        (window_size.0 as usize / 2).saturating_sub(width as usize + 1) as u16,
        (window_size.1 as usize / 2).saturating_sub(height as usize / 2) as u16,
    )
}

impl GameState {
    pub fn new(args: &Args) -> Self {
        let mut piece_bag = Bag::new();
        let width = args.horizontal;
        let height = args.vertical;
        let window_size = crossterm::terminal::size().map_or((10 as u16, 10 as u16), |e| e);

        let board_offset = get_board_offset(width, height, window_size);

        let initial_positon = ((width / 2) as u16, 2);

        let current_piece = CurrentPiece {
            piece: piece_bag.next(),
            x: initial_positon.0 as i32,
            y: initial_positon.1 as i32,
        };

        let mut next_board = Board {
            width: width + 2,
            height: height + 1,
            cells: vec![vec![0; (height + 1) as usize]; (width + 2) as usize],
        };
        let current_board = Board {
            width: width + 2,
            height: height + 1,
            cells: vec![vec![0; (height + 1) as usize]; (width + 2) as usize],
        };

        for i in 0..next_board.width {
            next_board.cells[i as usize][next_board.height.saturating_sub(1) as usize] = 8;
        }

        for i in 0..height {
            next_board.cells[0][i as usize] = 8;
            next_board.cells[(next_board.width.saturating_sub(1)) as usize][i as usize] = 8;
        }

        GameState {
            current_piece,
            next_piece: piece_bag.next(),
            next_board,
            current_board,
            piece_bag,
            lines: 0,
            level: 0,
            score: 0,
            show_tracer: false,
            show_next_piece: !args.hide_next_piece,
            startup_screen: true,
            window_size,
            board_offset,
            width,
            height,
            initial_positon,
            undo_used: false,
            pieces: 0,
        }
    }

    pub fn set_window_size(&mut self, window_size: (u16, u16)) {
        self.window_size = window_size;
        self.board_offset = get_board_offset(self.width, self.height, window_size);
        ()
    }

    pub fn toggle_tracer(&mut self) {
        self.show_tracer = !self.show_tracer;
    }

    pub fn toggle_next_piece(&mut self) {
        self.show_next_piece = !self.show_next_piece;
    }
}
