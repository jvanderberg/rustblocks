use std::thread;

use crate::{
    board::{piece_hit_bottom, update_board, Bag, Board, CurrentPiece},
    pieces::Piece,
};

#[derive(Clone, Debug)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Insane,
}

pub trait EventHandler {
    fn handle_event(&mut self, gs: &GameState, event: &GameEvent);
    fn clone_boxed(&self) -> Box<dyn EventHandler>;
}

impl Clone for Box<dyn EventHandler> {
    fn clone(&self) -> Box<dyn EventHandler> {
        self.clone_boxed()
    }
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

pub struct GameState {
    pub current_piece: CurrentPiece,
    pub next_piece: Piece,
    pub board: Board,
    pub difficulty: Difficulty,
    pub piece_bag: Bag,
    pub lines: i32,
    pub level: i32,
    pub score: i32,
    pub show_tracer: bool,
    pub show_next_piece: bool,
    pub startup_screen: bool,
    pub width: u16,
    pub height: u16,
    pub undo_used: bool,
    pub pieces: u16,
    pub game_over: bool,

    event_handlers: Vec<Box<dyn EventHandler>>,
}

impl Clone for GameState {
    fn clone(&self) -> Self {
        GameState {
            current_piece: self.current_piece.clone(),
            next_piece: self.next_piece.clone(),
            board: self.board.clone(),
            difficulty: self.difficulty.clone(),
            piece_bag: self.piece_bag.clone(),
            lines: self.lines,
            level: self.level,
            score: self.score,
            show_tracer: self.show_tracer,
            show_next_piece: self.show_next_piece,
            startup_screen: self.startup_screen,
            width: self.width,
            height: self.height,
            undo_used: self.undo_used,
            pieces: self.pieces,
            game_over: self.game_over,
            event_handlers: Vec::new(),
        }
    }
}
#[derive(Clone, Debug)]
pub enum GameEvent {
    ScoreChanged,
    LevelChanged,
    LinesClearedChanged,
    PieceMoved,
    PieceChanged,
    GameOver,
    GameReset,
    GameStarted,
}

fn get_initial_position(width: u16, _height: u16) -> (u16, u16) {
    ((width / 2) as u16, 2)
}

impl GameState {
    pub fn new(width: u16, height: u16, hide_next_piece: bool, difficulty: Difficulty) -> Self {
        let mut piece_bag = Bag::new();

        let initial_positon = get_initial_position(width, height);

        let current_piece = CurrentPiece {
            piece: piece_bag.next(),
            x: initial_positon.0 as i32,
            y: initial_positon.1 as i32,
        };

        let mut board = Board {
            width: width + 2,
            height: height + 1,
            cells: vec![vec![0; (height + 1) as usize]; (width + 2) as usize],
        };

        for i in 0..board.width {
            board.cells[i as usize][board.height.saturating_sub(1) as usize] = 8;
        }

        for i in 0..height {
            board.cells[0][i as usize] = 8;
            board.cells[(board.width.saturating_sub(1)) as usize][i as usize] = 8;
        }

        GameState {
            current_piece,
            next_piece: piece_bag.next(),
            board,
            difficulty,
            piece_bag,
            lines: 0,
            level: 0,
            score: 0,
            show_tracer: false,
            show_next_piece: !hide_next_piece,
            startup_screen: true,
            width,
            height,
            undo_used: false,
            pieces: 0,
            game_over: false,
            event_handlers: Vec::new(),
        }
    }

    pub fn restore(&self, old_state: &GameState) -> GameState {
        if self.game_over || self.pieces == 0 {
            return self.clone();
        }
        let mut gs = old_state.clone();
        gs.undo_used = true;
        gs.reset_current_piece();

        update_board(&mut gs);
        gs
    }
    pub fn start(&mut self) {
        self.startup_screen = false;
        self.emit(&GameEvent::GameStarted);
    }

    pub fn emit(&mut self, event: &GameEvent) {
        for handler in &mut self.event_handlers.clone() {
            handler.handle_event(self, event);
        }
    }

    pub fn add_event_handler<R>(&mut self, handler: Box<R>)
    where
        R: EventHandler + 'static,
    {
        self.event_handlers.push(handler);
    }

    pub fn get_initial_position(&self) -> (u16, u16) {
        get_initial_position(self.width, self.height)
    }

    ///
    /// Get the board offset from window size and board size
    /// this centers the board in the window
    ///

    pub fn toggle_tracer(&mut self) {
        self.show_tracer = !self.show_tracer;
    }

    pub fn toggle_show_next_piece(&mut self) {
        self.show_next_piece = !self.show_next_piece;
        self.emit(&GameEvent::PieceChanged);
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

    pub fn update_score(&mut self, lines_cleared: i32) {
        let lines = self.lines + lines_cleared;
        let level = (lines / 10) + 1;

        let score = self.score
            + match lines_cleared {
                1 => 100 * level,
                2 => 300 * level,
                3 => 500 * level,
                4 => 800 * level,
                _ => 0,
            };
        if self.lines != lines {
            self.lines = lines;
            self.emit(&GameEvent::LinesClearedChanged);
        }
        if self.level != level {
            self.level = level;
            self.emit(&GameEvent::LevelChanged);
        }
        if self.score != score {
            self.score = score;
            self.emit(&GameEvent::ScoreChanged);
        }
    }

    pub fn move_left(&mut self) -> bool {
        let res = !self.game_over && self.current_piece.move_left(&self.board);
        if res {
            update_board(self);
            self.emit(&GameEvent::PieceMoved);
        }
        res
    }

    pub fn move_right(&mut self) -> bool {
        let res = !self.game_over && self.current_piece.move_right(&self.board);
        if res {
            update_board(self);
            self.emit(&GameEvent::PieceMoved);
        }
        res
    }

    pub fn rotate_right(&mut self) -> bool {
        let res = !self.game_over && self.current_piece.rotate_right(&self.board);
        if res {
            update_board(self);
            self.emit(&GameEvent::PieceMoved);
        }
        res
    }
    pub fn move_down(&mut self) -> bool {
        let res = !self.game_over && self.current_piece.move_down(&self.board);
        if res {
            update_board(self);
            self.emit(&GameEvent::PieceMoved);
        }
        res
    }

    pub fn collides(&self) -> bool {
        self.current_piece
            .collides(&self.board, self.current_piece.x, self.current_piece.y)
    }

    ///
    /// Advance the game by one step, usually on a timer
    /// This will move the current piece down one step
    /// If the piece cannot move down, it will commit the piece to the board
    /// and spawn a new piece
    /// If the new piece collides with the board, the game is over
    ///
    pub fn advance_game(&mut self) -> bool {
        let success = self.current_piece.move_down(&self.board);
        update_board(self);
        if !success {
            let lines_cleared = piece_hit_bottom(self);

            self.update_score(lines_cleared);

            if self.collides() {
                // Game over
                self.game_over = true;
                self.emit(&GameEvent::GameOver);
            } else {
                update_board(self);
                self.emit(&GameEvent::PieceChanged);
            }
            false
        } else {
            update_board(self);
            self.emit(&GameEvent::PieceMoved);
            true
        }
    }

    ///
    /// Drop the current piece to the bottom of the board
    /// This will commit the piece to the board and spawn a new piece
    /// If the new piece collides with the board, the game is over
    ///
    pub fn drop(&mut self) {
        if self.game_over {
            return;
        }
        while self.current_piece.move_down(&self.board) {
            thread::sleep(std::time::Duration::from_millis(10));
            update_board(self);
            self.emit(&GameEvent::PieceMoved);
        }
        let lines_cleared = piece_hit_bottom(self);

        self.update_score(lines_cleared);
        if self.collides() {
            // Game over
            self.game_over = true;
            self.emit(&GameEvent::GameOver);
        } else {
            update_board(self);
            self.emit(&GameEvent::PieceChanged);
        }
    }

    pub fn reset_current_piece(&mut self) {
        // Touch up the backup state to reset the fallen piece to its original state
        self.current_piece = self.current_piece.clone();
        self.current_piece.x = self.get_initial_position().0 as i32;
        self.current_piece.y = self.get_initial_position().1 as i32;
        update_board(self);
        self.emit(&GameEvent::PieceChanged)
    }
}
