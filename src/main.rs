mod board;
mod gamestate;
mod pieces;
mod print;
use std::cell::RefCell;
use std::rc::Rc;

use board::initialize_board_pieces;
use clap::Parser;
use clap::{arg, command};
use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal,
};
use gamestate::{Difficulty, GameEvent, GameState};
use print::{hide_cursor, print_startup, show_cursor};

// macro_rules! event_handler {
//     ($renderer:expr, $gs: expr, $terminal_renderer:expr) => {
//         let event_handler = move |ge: &GameEvent, gs: &GameState| {
//             let tr = $terminal_renderer.clone();
//             match ge {
//                 GameEvent::ScoreChanged
//                 | GameEvent::LinesClearedChanged
//                 | GameEvent::LevelChanged
//                 | GameEvent::GameStarted => {
//                     tr.borrow_mut().print_score(&gs);
//                 }
//                 GameEvent::PieceChanged => {
//                     tr.borrow_mut()
//                         .draw_next_piece(&gs.next_piece, gs.show_next_piece);
//                     tr.borrow_mut()
//                         .draw_board(&gs.board, gs.current_piece.piece.color);
//                 }
//                 _ => {
//                     tr.borrow_mut()
//                         .draw_board(&gs.board, gs.current_piece.piece.color);
//                 }
//             }
//         };
//         $terminal_renderer = Rc::clone(&$renderer);
//         $gs.add_event_handler(event_handler);
//     };
// }

macro_rules! refresh_board {
    ( $gs: expr, $terminal_renderer:expr) => {
        $terminal_renderer.refresh_board(&$gs.board, $gs.current_piece.piece.color);
    };
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

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let mut gs = GameState::new(
        args.horizontal,
        args.vertical,
        args.hide_next_piece,
        args.difficulty.clone(),
    );

    let window_size = crossterm::terminal::size()?;

    //    let event_handler = get_handler(window_size.clone(), args.horizontal, args.vertical);
    let mut tr = print::TerminalRenderer::new(window_size, args.horizontal, args.vertical);

    // event_handler!(renderer, gs, terminal_renderer);
    gs.add_event_handler(Box::new(tr.clone()));
    initialize_board_pieces(&mut gs);
    let mut backup_state = gs.clone();

    let mut last_tick = std::time::SystemTime::now();

    terminal::enable_raw_mode()?;
    tr.clear_screen();
    hide_cursor();
    print_startup(1);

    loop {
        // Indicates if the board has changed and needs to be redrawn.
        let new_window_size = crossterm::terminal::size()?;

        let window_size = tr.get_window_size();
        if new_window_size != window_size {
            tr.set_window_size(new_window_size);
            refresh_board!(gs, tr);
            tr.print_score(&gs);
        }
        // Roughly eq to 60 frames per second, though in a terminal that makes little sense as
        // keyboard repeat rate plays the biggest role in the speed of the game.
        if poll(std::time::Duration::from_millis(16))? {
            let event = read()?;
            if gs.startup_screen {
                gs.startup_screen = false;
                refresh_board!(gs, tr);
                gs.start();
                continue;
            }

            let new_level = (gs.lines / 10) + 1;
            if new_level != gs.level {
                gs.level = new_level;
            }

            match event {
                Event::Key(KeyEvent {
                    kind: KeyEventKind::Press,
                    code,
                    modifiers: _,
                    state: _,
                }) => match code {
                    KeyCode::Esc => break,
                    KeyCode::Char('H') | KeyCode::Char('h') | KeyCode::Left => gs.move_left(),
                    KeyCode::Char('L') | KeyCode::Char('l') | KeyCode::Right => gs.move_right(),
                    KeyCode::Char('K') | KeyCode::Char('k') | KeyCode::Up => gs.rotate_right(),
                    KeyCode::Char('J') | KeyCode::Char('j') | KeyCode::Down => gs.move_down(),
                    KeyCode::Char('T') | KeyCode::Char('t') => {
                        gs.toggle_tracer();
                        true
                    }
                    KeyCode::Char('d') | KeyCode::Char('D') => {
                        gs.cycle_difficulty();
                        gs = GameState::new(
                            args.horizontal,
                            args.vertical,
                            args.hide_next_piece,
                            gs.difficulty.clone(),
                        );
                        gs.startup_screen = false;

                        gs.add_event_handler(Box::new(tr.clone()));
                        initialize_board_pieces(&mut gs);
                        gs.startup_screen = false;
                        last_tick = std::time::SystemTime::now();

                        backup_state = gs.clone();
                        refresh_board!(gs, tr);
                        gs.start();
                        continue;
                    }
                    KeyCode::Char('N') | KeyCode::Char('n') => {
                        gs.toggle_show_next_piece();
                        true
                    }
                    KeyCode::Char('U') | KeyCode::Char('u') => {
                        gs = gs.restore(&backup_state);
                        gs.add_event_handler(Box::new(tr.clone()));
                        gs.start();
                        // refresh_board!(gs, tr);
                        // tr.print_score(&gs);
                        // tr.draw_next_piece(&gs.next_piece, gs.show_next_piece);

                        true
                    }
                    KeyCode::Backspace | KeyCode::Delete => {
                        gs = GameState::new(
                            args.horizontal,
                            args.vertical,
                            args.hide_next_piece,
                            gs.difficulty.clone(),
                        );
                        gs.startup_screen = false;
                        gs.add_event_handler(Box::new(tr.clone()));
                        //event_handler!(renderer, gs, terminal_renderer);
                        initialize_board_pieces(&mut gs);
                        refresh_board!(gs, tr);

                        last_tick = std::time::SystemTime::now();

                        backup_state = gs.clone();
                        gs.start();
                        continue;
                    }
                    KeyCode::Char(' ') => {
                        backup_state = gs.clone();
                        gs.drop();

                        true
                    }
                    KeyCode::Char('q') => break,

                    _ => false,
                },

                _ => false,
            };
        }
        if gs.startup_screen {
            continue;
        }
        // Using unwrap here is safe because we know that the system time is always valid, if it's not, we have bigger problems.
        if !gs.game_over
            && last_tick.elapsed().unwrap().as_millis() > gs.get_piece_interval() as u128
        {
            last_tick = std::time::SystemTime::now();
            let temp_backup_state = gs.clone();
            if !gs.advance_game() {
                // The piece has hit bottom, snapshot the state before it fell
                backup_state = temp_backup_state;
            }
        }
    }

    terminal::disable_raw_mode()?;
    show_cursor();
    Ok(())
}
