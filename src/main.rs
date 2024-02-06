use std::{io::stdout, thread};
mod board;
mod pieces;
mod print;
mod score;
use board::{clear_lines, commit_piece, update_board, Bag, Board, CurrentPiece};
use clap::{arg, command, Parser};
use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal, ExecutableCommand,
};
use pieces::Piece;
use print::{print_next_piece, print_startup};
use score::calc_score;

fn piece_hit_bottom(
    current_piece: &mut CurrentPiece,
    next_board: &mut Board,
    next_piece: Piece,
    lines: i32,
    score: i32,
    piece_bag: &mut Bag,
    initial_positon: (i32, i32),
) -> (CurrentPiece, Piece, i32, i32, i32) {
    commit_piece(
        &current_piece.piece,
        next_board,
        current_piece.x,
        current_piece.y,
        current_piece.piece.color,
    );

    let (lines, score, level) = calc_score(clear_lines(next_board), lines, score);

    print_next_piece(&piece_bag.peek(), &next_piece);
    (
        CurrentPiece {
            piece: next_piece,
            x: initial_positon.0,
            y: initial_positon.1,
        },
        piece_bag.next(),
        lines,
        score,
        level,
    )
}
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None,)]
struct Args {
    /// The width of the board
    #[arg(short = 'w', long, default_value = "10")]
    horizontal: u8,
    /// The height of the board
    #[arg(short, long, default_value = "22")]
    vertical: u8,
}
fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let width = args.horizontal as usize;
    let height = args.vertical as usize;

    let mut startup_screen = true;
    let mut show_tracer = false;
    let mut piece_bag = Bag::new();
    let mut lines = 0;
    let mut level = 1;
    let mut score = 0;

    let window_size = crossterm::terminal::size()?;

    let board_offet = (
        window_size.0 as usize / 2 - width - 1,
        window_size.1 as usize / 2 - height / 2,
    );

    let initial_positon = ((width / 2) as i32, 2);
    let mut last_tick = std::time::SystemTime::now();
    let mut current_piece = CurrentPiece {
        piece: piece_bag.next(),
        x: initial_positon.0,
        y: initial_positon.1,
    };

    let mut next_piece: Piece = piece_bag.next();

    let mut next_board = Board {
        width: width + 2,
        height: height + 1,
        cells: vec![vec![0; height + 1]; width + 2],
    };
    let mut current_board = Board {
        width: width + 2,
        height: height + 1,
        cells: vec![vec![0; height + 1]; width + 2],
    };

    for i in 0..next_board.width {
        next_board.cells[i][next_board.height - 1] = 8;
    }

    for i in 0..height {
        next_board.cells[0][i] = 8;
        next_board.cells[next_board.width - 1][i] = 8;
    }
    terminal::enable_raw_mode()?;
    let _ = stdout()
        .execute(terminal::Clear(terminal::ClearType::All))?
        .execute(cursor::Hide);

    print_startup(1);

    loop {
        // Indicates if the board has changed and needs to be redrawn.
        let mut changed = false;

        // Roughly eq to 60 frames per second, though in a terminal that makes little sense as
        // keyboard repeat rate plays the biggest role in the speed of the game.
        if poll(std::time::Duration::from_millis(16))? {
            let event = read()?;
            if startup_screen {
                startup_screen = false;
                let _ = stdout()
                    .execute(terminal::Clear(terminal::ClearType::All))?
                    .execute(cursor::Hide);
                update_board(
                    &current_piece,
                    &mut current_board,
                    &mut next_board,
                    show_tracer,
                    board_offet,
                );
                (lines, score, level) = calc_score(clear_lines(&mut next_board), lines, score);
                print_next_piece(&next_piece, &current_piece.piece);

                continue;
            }

            let new_level = (lines / 10) + 1;
            if new_level != level {
                level = new_level;
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
                        current_piece.move_left(&current_board)
                    }
                    KeyCode::Char('L') | KeyCode::Char('l') | KeyCode::Right => {
                        current_piece.move_right(&current_board)
                    }
                    KeyCode::Char('K') | KeyCode::Char('k') | KeyCode::Up => {
                        current_piece.rotate_right(&current_board)
                    }
                    KeyCode::Char('J') | KeyCode::Char('j') | KeyCode::Down => {
                        current_piece.move_down(&current_board)
                    }
                    KeyCode::Char('T') | KeyCode::Char('t') => {
                        show_tracer = !show_tracer;
                        true
                    }
                    KeyCode::Char(' ') => {
                        while current_piece.move_down(&current_board) {
                            thread::sleep(std::time::Duration::from_millis(10));
                            update_board(
                                &current_piece,
                                &mut current_board,
                                &mut next_board,
                                false,
                                board_offet,
                            );
                        }

                        (current_piece, next_piece, lines, score, level) = piece_hit_bottom(
                            &mut current_piece,
                            &mut next_board,
                            next_piece,
                            lines,
                            score,
                            &mut piece_bag,
                            initial_positon,
                        );
                        if current_piece.collides(&next_board, current_piece.x, current_piece.y) {
                            break;
                        }

                        true
                    }
                    KeyCode::Char('q') => break,

                    _ => true,
                },

                _ => false,
            }
        };
        if startup_screen {
            continue;
        }

        let mut interval = 1000 - level * 50;
        if (interval as i32) < 250 {
            interval = 250;
        }

        // Using unwrap here is safe because we know that the system time is always valid, if it's not, we have bigger problems.
        if last_tick.elapsed().unwrap().as_millis() > interval as u128 {
            last_tick = std::time::SystemTime::now();
            let success = current_piece.move_down(&current_board);
            if !success {
                // We've hit the bottom, so we need to draw the piece permanently on the board and get a new piece.

                (current_piece, next_piece, lines, score, level) = piece_hit_bottom(
                    &mut current_piece,
                    &mut next_board,
                    next_piece,
                    lines,
                    score,
                    &mut piece_bag,
                    initial_positon,
                );

                if current_piece.collides(&next_board, current_piece.x, current_piece.y) {
                    break;
                }
            }
            changed = true;
        }
        if changed {
            update_board(
                &current_piece,
                &mut current_board,
                &mut next_board,
                show_tracer,
                board_offet,
            );
        }
    }
    terminal::disable_raw_mode()?;
    let _ = stdout().execute(cursor::Show);
    Ok(())
}
