use std::thread;

use crate::{
    board::{
        clear_lines, commit_current_piece, commit_piece, draw_tracer, remove_tracer, Bag, Board,
        CurrentPiece,
    },
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
#[derive(Clone, Debug, PartialEq)]
pub enum GameStatus {
    Running,
    Paused,
    GameOver,
    NotStarted,
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

///
/// Simple macro to DRY-up the move logic
/// This macro will move the current piece in the direction of the command
/// If the piece can move, it will update the board and emit the PieceMoved event
/// If the piece cannot move, it will return false
///
macro_rules! move_piece {
    ($gs: expr, $command: ident ) => {
        let res = $gs.status == GameStatus::Running && $gs.current_piece.$command(&$gs.board);
        if res {
            $gs.update_board();
            $gs.emit(&GameEvent::PieceMoved);
        }
        return res;
    };
}

pub struct GameState {
    current_piece: CurrentPiece,
    next_piece: Piece,
    board: Board,
    difficulty: Difficulty,
    piece_bag: Bag,
    lines: i32,
    level: i32,
    score: i32,
    show_tracer: bool,
    show_next_piece: bool,
    width: u16,
    height: u16,
    undo_used: bool,
    pieces: u16,
    status: GameStatus,
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
            width: self.width,
            height: self.height,
            undo_used: self.undo_used,
            pieces: self.pieces,
            status: self.status.clone(),
            event_handlers: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
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
            width,
            height,
            undo_used: false,
            pieces: 0,
            status: GameStatus::NotStarted,
            event_handlers: Vec::new(),
        }
    }

    pub fn get_difficulty(&self) -> Difficulty {
        self.difficulty.clone()
    }

    pub fn get_next_piece(&self) -> Piece {
        self.next_piece.clone()
    }
    ///
    /// Get score, lines, and level
    ///
    pub fn get_score(&self) -> (i32, i32, i32) {
        (self.score, self.lines, self.level)
    }

    pub fn get_status(&self) -> &GameStatus {
        &self.status
    }

    pub fn get_current_piece(&self) -> &CurrentPiece {
        &self.current_piece
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn get_show_next_piece(&self) -> bool {
        self.show_next_piece
    }

    pub fn get_show_tracer(&self) -> bool {
        self.show_tracer
    }

    pub fn get_undo_used(&self) -> bool {
        self.undo_used
    }

    ///
    /// Restore from a previous game state, useful for implementing save/restore or undo
    /// This will restore the game state to the previous state, including the
    /// current piece, the board, and the next piece
    /// If the game is over, or there are no pieces left, it will return the
    /// current state
    ///
    pub fn restore(&self, old_state: &GameState) -> GameState {
        let mut gs = old_state.clone();
        gs.undo_used = true;
        gs.reset_current_piece();
        gs.update_board();
        gs
    }

    ///
    /// Start, or restart the game
    ///
    pub fn start(&mut self) {
        self.status = GameStatus::Running;
        self.emit(&GameEvent::GameStarted);
    }

    ///
    /// Emit an event to all event handlers
    ///
    pub fn emit(&mut self, event: &GameEvent) {
        for handler in &mut self.event_handlers.clone() {
            handler.handle_event(self, event);
        }
    }

    ///
    /// Add an event handler to the game state
    ///
    pub fn add_event_handler<R>(&mut self, handler: Box<R>)
    where
        R: EventHandler + 'static,
    {
        self.event_handlers.push(handler);
    }

    ///
    /// Get the initial position for a new piece
    ///
    pub fn get_initial_position(&self) -> (u16, u16) {
        get_initial_position(self.width, self.height)
    }

    ///
    /// Toggle the tracer block that tells you where a piece will fall
    ///
    pub fn toggle_tracer(&mut self) {
        self.show_tracer = !self.show_tracer;
    }

    ///
    /// Toggle the display of the next piece
    ///
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

    ///
    /// Cycle the difficulty level
    /// In the current game it will increase the speed.
    /// On reset, on the hardest levels it will draw random blocks
    ///
    pub fn cycle_difficulty(&mut self) {
        self.difficulty = match self.difficulty {
            Difficulty::Easy => Difficulty::Medium,
            Difficulty::Medium => Difficulty::Hard,
            Difficulty::Hard => Difficulty::Insane,
            Difficulty::Insane => Difficulty::Easy,
        };
    }

    ///
    /// Calculate the new score
    ///
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

    ///
    /// Move the current piece left
    ///
    pub fn move_left(&mut self) -> bool {
        move_piece!(self, move_left);
    }

    ///
    /// Move the current piece right
    ///
    pub fn move_right(&mut self) -> bool {
        move_piece!(self, move_right);
    }

    ///
    /// Rotate the current piece right
    ///
    pub fn rotate_right(&mut self) -> bool {
        move_piece!(self, rotate_right);
    }

    ///
    /// Move the current piece down
    ///
    pub fn move_down(&mut self) -> bool {
        move_piece!(self, move_down);
    }

    ///
    /// Test if the currrent piece collides with the board pieces
    ///
    pub fn collides(&self) -> bool {
        self.current_piece
            .collides(&self.board, self.current_piece.x, self.current_piece.y)
    }

    ///
    /// Advance the game by one step, usually on a timer
    /// This will move the current piece down one step
    /// If the piece cannot move down, it will commit the piece to the board
    /// and spawn a new piece
    /// If the new piece collides with the board, the game is over,
    /// Ideally this should be called every get_piece_interval() milliseconds, though an
    /// implementor can ignore that and call it as often as they like
    ///
    pub fn advance_game(&mut self) -> bool {
        let success = self.current_piece.move_down(&self.board);
        self.update_board();
        if !success {
            let lines_cleared = self.piece_hit_bottom();

            self.update_score(lines_cleared);

            if self.collides() {
                // Game over
                self.status = GameStatus::GameOver;
                self.emit(&GameEvent::GameOver);
            } else {
                self.update_board();
                self.emit(&GameEvent::PieceChanged);
            }
            false
        } else {
            self.update_board();
            self.emit(&GameEvent::PieceMoved);
            true
        }
    }

    ///
    /// Drop the current piece to the bottom of the board
    /// This will commit the piece to the board and spawn a new piece
    /// If the new piece collides with the board, the game is over
    ///
    pub fn drop(&mut self) -> bool {
        if self.status != GameStatus::Running {
            return false;
        }
        while self.current_piece.move_down(&self.board) {
            thread::sleep(std::time::Duration::from_millis(10));
            self.update_board();
            self.emit(&GameEvent::PieceMoved);
        }
        let lines_cleared = self.piece_hit_bottom();

        self.update_score(lines_cleared);
        if self.collides() {
            // Game over
            self.status = GameStatus::GameOver;
            self.emit(&GameEvent::GameOver);
            false
        } else {
            self.update_board();
            self.emit(&GameEvent::PieceChanged);
            true
        }
    }

    ///
    /// This resets the current piece to the iniatial position
    /// Useful for undo, or restoring a game
    ///
    pub fn reset_current_piece(&mut self) {
        self.current_piece = self.current_piece.clone();
        self.current_piece.x = self.get_initial_position().0 as i32;
        self.current_piece.y = self.get_initial_position().1 as i32;
        self.update_board();
        self.emit(&GameEvent::PieceChanged)
    }

    ///
    /// Updates the board after a change
    ///
    pub fn update_board(self: &mut GameState) {
        commit_current_piece(&self.current_piece, &mut self.board);

        if self.show_tracer {
            let mut tracer = self.current_piece.clone();
            while tracer.move_down(&self.board) {}
            draw_tracer(&tracer.piece, &mut self.board, tracer.x, tracer.y);
        } else {
            remove_tracer(&mut self.board);
        }
    }

    ///
    /// Handles the piece hitting the bottom of the board.
    /// This includes committing the piece to the board, clearing any lines, and updating the score.
    /// It also sets the next piece to the current piece and gets a new next piece from the piece bag, and
    /// prints the new next piece on the board.
    ///

    pub fn piece_hit_bottom(self: &mut GameState) -> i32 {
        // Commit the piece to the board with it's actual color, not the '255' current piece color.
        // This makes it 'permanent' on the board.
        commit_piece(
            &self.current_piece.piece,
            &mut self.board,
            self.current_piece.x,
            self.current_piece.y,
            self.current_piece.piece.color,
        );
        self.pieces += 1;
        let lines = clear_lines(&mut self.board);

        self.current_piece = CurrentPiece {
            piece: self.next_piece.clone(),
            x: self.get_initial_position().0 as i32,
            y: self.get_initial_position().1 as i32,
        };
        self.next_piece = self.piece_bag.next();
        return lines;
    }

    ///
    ///  Based on the difficulty, we want to introduce some random pieces, move them randomly, and drop them
    /// to make the game more interesting.
    ///
    pub fn initialize_board_pieces(self: &mut GameState) {
        let extra_pieces = match self.difficulty {
            Difficulty::Easy | Difficulty::Medium => 0,
            Difficulty::Hard => 5,
            Difficulty::Insane => 10,
        };
        for _i in 0..extra_pieces {
            let mut piece: CurrentPiece = CurrentPiece {
                piece: self.piece_bag.next().clone(),
                x: self.get_initial_position().0 as i32,
                y: self.get_initial_position().1 as i32,
            };

            piece.rotate_right(&self.board);
            // Randomly move the piece left or right
            let int = rand::random::<i32>() % (self.width - self.width / 2) as i32;
            if int > 0 {
                for _ in 0..int {
                    piece.move_left(&self.board);
                }
            } else {
                for _ in 0..int.abs() {
                    piece.move_right(&self.board);
                }
            }

            while piece.move_down(&self.board) {}
            commit_piece(
                &piece.piece,
                &mut self.board,
                piece.x,
                piece.y,
                piece.piece.color,
            );
            self.update_board();
        }

        for i in 0..self.board.width {
            self.board.cells[i as usize][self.board.height.saturating_sub(1) as usize] = 8;
        }

        for i in 0..self.height {
            self.board.cells[0][i as usize] = 8;
            self.board.cells[(self.board.width.saturating_sub(1)) as usize][i as usize] = 8;
        }
    }
}
