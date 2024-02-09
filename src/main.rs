use std::thread;
mod board;
mod gamestate;
mod pieces;
mod print;
mod score;
use board::{
    clear_board, hide_cursor, initialize_board_pieces, piece_hit_bottom, refresh_board,
    show_cursor, update_board,
};
use clap::Parser;
use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal,
};
use gamestate::{Args, GameState};
use print::print_startup;

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let mut gs = GameState::new(&args, None);
    initialize_board_pieces(&mut gs);
    let mut backup_state = gs.clone();

    let mut last_tick = std::time::SystemTime::now();

    terminal::enable_raw_mode()?;
    clear_board();
    hide_cursor();
    print_startup(1);

    let mut piece_changed = true;
    loop {
        // Indicates if the board has changed and needs to be redrawn.
        let mut changed = false;
        let new_window_size = crossterm::terminal::size()?;
        if new_window_size != gs.window_size {
            gs.window_size = new_window_size;
            refresh_board(&mut gs);
            continue;
        }
        // Roughly eq to 60 frames per second, though in a terminal that makes little sense as
        // keyboard repeat rate plays the biggest role in the speed of the game.
        if poll(std::time::Duration::from_millis(16))? {
            piece_changed = false;
            let event = read()?;
            if gs.startup_screen {
                gs.startup_screen = false;
                refresh_board(&mut gs);
                continue;
            }

            let new_level = (gs.lines / 10) + 1;
            if new_level != gs.level {
                gs.level = new_level;
            }

            changed = match event {
                Event::Key(KeyEvent {
                    kind: KeyEventKind::Press,
                    code,
                    modifiers: _,
                    state: _,
                }) => match code {
                    KeyCode::Esc => break,
                    KeyCode::Char('H') | KeyCode::Char('h') | KeyCode::Left => {
                        !gs.game_over && gs.current_piece.move_left(&gs.current_board)
                    }
                    KeyCode::Char('L') | KeyCode::Char('l') | KeyCode::Right => {
                        !gs.game_over && gs.current_piece.move_right(&gs.current_board)
                    }
                    KeyCode::Char('K') | KeyCode::Char('k') | KeyCode::Up => {
                        !gs.game_over && gs.current_piece.rotate_right(&gs.current_board)
                    }
                    KeyCode::Char('J') | KeyCode::Char('j') | KeyCode::Down => {
                        !gs.game_over && gs.current_piece.move_down(&gs.current_board)
                    }
                    KeyCode::Char('T') | KeyCode::Char('t') => {
                        gs.toggle_tracer();
                        true
                    }
                    KeyCode::Char('d') | KeyCode::Char('D') => {
                        gs.cycle_difficulty();
                        gs = GameState::new(&args, Some(gs.difficulty.clone()));
                        initialize_board_pieces(&mut gs);
                        gs.startup_screen = false;
                        last_tick = std::time::SystemTime::now();
                        refresh_board(&mut gs);
                        backup_state = gs.clone();
                        continue;
                    }
                    KeyCode::Char('N') | KeyCode::Char('n') => {
                        gs.toggle_next_piece();
                        piece_changed = true;
                        true
                    }
                    KeyCode::Char('U') | KeyCode::Char('u') => {
                        if gs.game_over || gs.pieces == 0 {
                            // Ignore if there are no pieces already committed to the board
                            continue;
                        }
                        piece_changed = true;
                        gs = backup_state.clone();
                        gs.undo_used = true;
                        refresh_board(&mut gs);
                        true
                    }
                    KeyCode::Backspace | KeyCode::Delete => {
                        gs = GameState::new(&args, Some(gs.difficulty.clone()));
                        initialize_board_pieces(&mut gs);
                        gs.startup_screen = false;
                        last_tick = std::time::SystemTime::now();
                        refresh_board(&mut gs);
                        backup_state = gs.clone();
                        continue;
                    }
                    KeyCode::Char(' ') => {
                        if gs.game_over {
                            continue;
                        }
                        while gs.current_piece.move_down(&gs.current_board) {
                            thread::sleep(std::time::Duration::from_millis(10));
                            update_board(&mut gs, false);
                        }
                        backup_state = gs.clone();

                        piece_hit_bottom(&mut gs, &mut backup_state);
                        if gs.current_piece.collides(
                            &gs.next_board,
                            gs.current_piece.x,
                            gs.current_piece.y,
                        ) {
                            gs.game_over = true;
                        }
                        piece_changed = true;
                        true
                    }
                    KeyCode::Char('q') => break,

                    _ => true,
                },

                _ => false,
            }
        };
        if gs.startup_screen {
            continue;
        }

        // Using unwrap here is safe because we know that the system time is always valid, if it's not, we have bigger problems.
        if !gs.game_over
            && last_tick.elapsed().unwrap().as_millis() > gs.get_piece_interval() as u128
        {
            last_tick = std::time::SystemTime::now();
            let success = gs.current_piece.move_down(&gs.current_board);
            if !success {
                // We've hit the bottom, so we need to draw the piece permanently on the board and get a new piece.
                backup_state = gs.clone();
                piece_hit_bottom(&mut gs, &mut backup_state);

                if gs
                    .current_piece
                    .collides(&gs.next_board, gs.current_piece.x, gs.current_piece.y)
                {
                    // Game over
                    gs.game_over = true;
                }
                piece_changed = true;
            }
            changed = true;
        }
        if changed && !gs.game_over {
            update_board(&mut gs, piece_changed);
        }
    }
    terminal::disable_raw_mode()?;
    show_cursor();
    Ok(())
}
