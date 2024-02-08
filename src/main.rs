use std::{io::stdout, thread};
mod board;
mod gamestate;
mod pieces;
mod print;
mod score;
use board::{clear_lines, piece_hit_bottom, update_board, Board};
use clap::Parser;
use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal, ExecutableCommand,
};
use gamestate::{Args, GameState};
use print::print_startup;
use score::update_score;

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let mut gs = GameState::new(&args);
    let mut backup_state = gs.clone();

    let mut last_tick = std::time::SystemTime::now();

    terminal::enable_raw_mode()?;
    let _ = stdout()
        .execute(terminal::Clear(terminal::ClearType::All))?
        .execute(cursor::Hide);

    print_startup(1);

    let mut piece_changed = true;
    loop {
        // Indicates if the board has changed and needs to be redrawn.
        let mut changed = false;
        let new_window_size = crossterm::terminal::size()?;
        if new_window_size != gs.window_size {
            gs.set_window_size(new_window_size);
            let _ = stdout()
                .execute(terminal::Clear(terminal::ClearType::All))
                .unwrap();
            gs.current_board = Board {
                width: gs.width + 2,
                height: gs.height + 1,
                cells: vec![vec![0; gs.height as usize + 1]; gs.width as usize + 2],
            };
            update_board(&mut gs, true);
            let lines_cleared = clear_lines(&mut gs);
            update_score(&mut gs, lines_cleared);

            continue;
        }
        // Roughly eq to 60 frames per second, though in a terminal that makes little sense as
        // keyboard repeat rate plays the biggest role in the speed of the game.
        if poll(std::time::Duration::from_millis(16))? {
            piece_changed = false;
            let event = read()?;
            if gs.startup_screen {
                gs.startup_screen = false;
                let _ = stdout()
                    .execute(terminal::Clear(terminal::ClearType::All))?
                    .execute(cursor::Hide);
                update_board(&mut gs, true);
                let lines_cleared = clear_lines(&mut gs);
                update_score(&mut gs, lines_cleared);

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
                        gs.current_piece.move_left(&gs.current_board)
                    }
                    KeyCode::Char('L') | KeyCode::Char('l') | KeyCode::Right => {
                        gs.current_piece.move_right(&gs.current_board)
                    }
                    KeyCode::Char('K') | KeyCode::Char('k') | KeyCode::Up => {
                        gs.current_piece.rotate_right(&gs.current_board)
                    }
                    KeyCode::Char('J') | KeyCode::Char('j') | KeyCode::Down => {
                        gs.current_piece.move_down(&gs.current_board)
                    }
                    KeyCode::Char('T') | KeyCode::Char('t') => {
                        gs.toggle_tracer();
                        true
                    }
                    KeyCode::Char('N') | KeyCode::Char('n') => {
                        gs.toggle_next_piece();
                        piece_changed = true;
                        true
                    }
                    KeyCode::Char('U') | KeyCode::Char('u') => {
                        if gs.pieces == 0 {
                            // Ignore if there are no pieces already committed to the board
                            continue;
                        }
                        piece_changed = true;
                        gs = backup_state.clone();
                        gs.undo_used = true;
                        let _ = stdout()
                            .execute(terminal::Clear(terminal::ClearType::All))
                            .unwrap();
                        gs.current_board = Board {
                            width: gs.width + 2,
                            height: gs.height + 1,
                            cells: vec![vec![0; gs.height as usize + 1]; gs.width as usize + 2],
                        };
                        update_score(&mut gs, 0);
                        true
                    }
                    KeyCode::Backspace | KeyCode::Delete => {
                        gs = GameState::new(&args);
                        gs.startup_screen = false;
                        last_tick = std::time::SystemTime::now();

                        let _ = stdout()
                            .execute(terminal::Clear(terminal::ClearType::All))
                            .unwrap();
                        gs.current_board = Board {
                            width: gs.width + 2,
                            height: gs.height + 1,
                            cells: vec![vec![0; gs.height as usize + 1]; gs.width as usize + 2],
                        };
                        backup_state = gs.clone();
                        update_board(&mut gs, true);
                        update_score(&mut gs, 0);
                        continue;
                    }
                    KeyCode::Char(' ') => {
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
                            break;
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

        let mut interval = 1000 - gs.level * 50;
        if (interval as i32) < 250 {
            interval = 250;
        }

        // Using unwrap here is safe because we know that the system time is always valid, if it's not, we have bigger problems.
        if last_tick.elapsed().unwrap().as_millis() > interval as u128 {
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
                    break;
                }
                piece_changed = true;
            }
            changed = true;
        }
        if changed {
            update_board(&mut gs, piece_changed);
        }
    }
    terminal::disable_raw_mode()?;
    let _ = stdout().execute(cursor::Show);
    Ok(())
}
