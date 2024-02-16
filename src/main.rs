mod print;

use blocks_lib::gamestate::DropSpeed;
use blocks_lib::gamestate::{Difficulty, GameEvent, GameState, GameStatus};
use clap::Parser;
use clap::{arg, command};
use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal,
};
use print::{hide_cursor, print_startup, show_cursor};

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
    let color;
    match terminal_light::luma() {
        Ok(luma) if luma > 0.85 => {
            // Use a "light mode" skin.
            color = 0;
        }
        Ok(luma) if luma < 0.2 => {
            // Use a "dark mode" skin.
            color = 15;
        }
        _ => {
            color = 93;
        }
    }
    let args = Args::parse();

    let window_size = crossterm::terminal::size()?;

    let tr = print::TerminalRenderer::new(window_size, args.horizontal, args.vertical, color);

    let mut gs = GameState::new(
        args.horizontal,
        args.vertical,
        args.hide_next_piece,
        args.difficulty.clone(),
    );
    let ev = |ge: &GameEvent, gs: &GameState| match ge {
        GameEvent::ScoreChanged | GameEvent::LinesClearedChanged | GameEvent::LevelChanged => {
            tr.draw_score(&gs);
        }
        GameEvent::GameStarted | GameEvent::GameReset => {
            tr.refresh_board(&gs);
        }
        GameEvent::PieceChanged => {
            tr.draw_next_piece(&gs.get_next_piece(), gs.get_show_next_piece());
            tr.draw_board(&gs.get_board());
        }
        _ => {
            tr.draw_board(&gs.get_board());
        }
    };
    gs.add_event_handler(&ev);

    let mut last_tick = std::time::SystemTime::now();

    terminal::enable_raw_mode()?;
    tr.clear_screen();
    hide_cursor();
    print_startup(color);

    loop {
        // Indicates if the board has changed and needs to be redrawn.
        let new_window_size = crossterm::terminal::size()?;

        let window_size = tr.get_window_size();
        if new_window_size != window_size {
            tr.set_window_size(new_window_size);
            tr.refresh_board(&gs);
        }
        // Roughly eq to 60 frames per second, though in a terminal that makes little sense as
        // keyboard repeat rate plays the biggest role in the speed of the game.
        if poll(std::time::Duration::from_millis(16))? {
            let event = read()?;
            if *gs.get_status() == GameStatus::NotStarted {
                // tr.refresh_board(&gs);
                gs.start();
                continue;
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
                            gs.get_difficulty().clone(),
                        );
                        gs.add_event_handler(&ev);
                        gs.start();
                        last_tick = std::time::SystemTime::now();

                        continue;
                    }
                    KeyCode::Char('N') | KeyCode::Char('n') => {
                        gs.toggle_show_next_piece();
                        true
                    }
                    KeyCode::Char('U') | KeyCode::Char('u') => {
                        gs = gs.undo();
                        gs.add_event_handler(&ev);
                        gs.start();
                        true
                    }
                    KeyCode::Backspace | KeyCode::Delete => {
                        gs = GameState::new(
                            args.horizontal,
                            args.vertical,
                            args.hide_next_piece,
                            gs.get_difficulty().clone(),
                        );
                        gs.add_event_handler(&ev);
                        gs.start();

                        last_tick = std::time::SystemTime::now();

                        continue;
                    }
                    KeyCode::Char(' ') => {
                        gs.drop(DropSpeed::default());

                        true
                    }
                    KeyCode::Char('q') => break,

                    _ => false,
                },

                _ => false,
            };
        }
        if *gs.get_status() == GameStatus::NotStarted {
            continue;
        }
        // Using unwrap here is safe because we know that the system time is always valid, if it's not, we have bigger problems.
        if (*gs.get_status() == GameStatus::Running)
            && last_tick.elapsed().unwrap().as_millis() > gs.get_piece_interval() as u128
        {
            last_tick = std::time::SystemTime::now();
            gs.advance_game();
        }
    }

    terminal::disable_raw_mode()?;
    show_cursor();
    Ok(())
}
