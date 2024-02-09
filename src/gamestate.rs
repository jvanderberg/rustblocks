use crate::{
    board::{Bag, Board, CurrentPiece},
    pieces::Piece,
};
use clap::{arg, command, Parser};

#[derive(Clone, Debug)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Insane,
}

impl std::str::FromStr for Difficulty {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Easy" | "easy" | "1" => Ok(Difficulty::Easy),
            "Medium" | "medium" | "2" => Ok(Difficulty::Medium),
            "Hard" | "hard" | "3" => Ok(Difficulty::Hard),
            "Insane" | "insane" | "4" => Ok(Difficulty::Insane),
            _ => Err("Invalid difficulty".to_string()),
        }
    }
}

impl std::string::ToString for Difficulty {
    fn to_string(&self) -> String {
        match self {
            Difficulty::Easy => "Easy".to_string(),
            Difficulty::Medium => "Medium".to_string(),
            Difficulty::Hard => "Hard".to_string(),
            Difficulty::Insane => "Insane".to_string(),
        }
    }
}
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

    /// The difficulty of the game, changes the speed of the game.
    /// Easy, Medium, Hard, Insane, or 1, 2, 3, 4
    #[arg(short, long, default_value = "Easy")]
    difficulty: Difficulty,
}

#[derive(Clone)]
pub struct GameState {
    pub current_piece: CurrentPiece,
    pub next_piece: Piece,
    pub next_board: Board,
    pub current_board: Board,
    pub difficulty: Difficulty,
    pub piece_bag: Bag,
    pub lines: i32,
    pub level: i32,
    pub score: i32,
    pub show_tracer: bool,
    pub show_next_piece: bool,
    pub startup_screen: bool,
    pub window_size: (u16, u16),
    pub width: u16,
    pub height: u16,
    pub undo_used: bool,
    pub pieces: u16,
    pub game_over: bool,
}

fn get_initial_position(width: u16, _height: u16) -> (u16, u16) {
    ((width / 2) as u16, 2)
}

impl GameState {
    pub fn new(args: &Args, difficulty: Option<Difficulty>) -> Self {
        let mut piece_bag = Bag::new();
        let width = args.horizontal;
        let height = args.vertical;
        let difficulty = match difficulty {
            Some(d) => d,
            None => args.difficulty.clone(),
        };

        let window_size = crossterm::terminal::size().map_or((10 as u16, 10 as u16), |e| e);

        let initial_positon = get_initial_position(width, height);

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
            difficulty,
            piece_bag,
            lines: 0,
            level: 0,
            score: 0,
            show_tracer: false,
            show_next_piece: !args.hide_next_piece,
            startup_screen: true,
            window_size,
            width,
            height,
            undo_used: false,
            pieces: 0,
            game_over: false,
        }
    }

    pub fn get_initial_position(&self) -> (u16, u16) {
        get_initial_position(self.width, self.height)
    }

    ///
    /// Get the board offset from window size and board size
    /// this centers the board in the window
    ///
    pub fn get_board_offset(&self) -> (u16, u16) {
        (
            (self.window_size.0 as usize / 2).saturating_sub(self.width as usize + 1) as u16,
            (self.window_size.1 as usize / 2).saturating_sub(self.height as usize / 2) as u16,
        )
    }

    pub fn toggle_tracer(&mut self) {
        self.show_tracer = !self.show_tracer;
    }

    pub fn toggle_next_piece(&mut self) {
        self.show_next_piece = !self.show_next_piece;
    }

    ///
    /// The interval in ms for each automatic gravity drop
    ///
    pub fn get_piece_interval(&self) -> u16 {
        let bounds = match self.difficulty {
            Difficulty::Easy => (1000, 200),
            Difficulty::Medium => (800, 150),
            Difficulty::Hard => (700, 100),
            Difficulty::Insane => (500, 50),
        };

        let interval = bounds.0 - &self.level * 50;
        if (interval as i32) < bounds.1 {
            return bounds.1 as u16;
        }
        interval as u16
    }

    pub fn cycle_difficulty(&mut self) {
        self.difficulty = match self.difficulty {
            Difficulty::Easy => Difficulty::Medium,
            Difficulty::Medium => Difficulty::Hard,
            Difficulty::Hard => Difficulty::Insane,
            Difficulty::Insane => Difficulty::Easy,
        };
    }
}
