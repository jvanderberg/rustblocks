//printstuff

pub fn hide_cursor() {
    stdout().execute(cursor::Hide).unwrap();
}

pub fn show_cursor() {
    stdout().execute(cursor::Show).unwrap();
}
pub fn clear_board() {
    stdout()
        .execute(terminal::Clear(terminal::ClearType::All))
        .unwrap();
}
pub fn update_score(gs: &mut GameState, lines_cleared: i32) {
    gs.lines = gs.lines + lines_cleared;
    let new_level = (gs.lines / 10) + 1;

    gs.score = gs.score
        + match lines_cleared {
            1 => 100 * new_level,
            2 => 300 * new_level,
            3 => 500 * new_level,
            4 => 800 * new_level,
            _ => 0,
        };

    print_xy(
        3,
        1,
        Color::AnsiValue(1),
        gs.difficulty.to_string().as_str(),
        (0, 0),
    );
    print_xy(
        3 + gs.difficulty.to_string().len() as u16 + 1,
        1,
        Color::AnsiValue(1),
        "Mode",
        (0, 0),
    );
    let score_text = if gs.undo_used {
        "Score (Undo Used)"
    } else {
        "Score"
    };
    print_xy(3, 3, Color::AnsiValue(1), score_text, (0, 0));
    print_xy(
        3,
        4,
        Color::AnsiValue(1),
        format!("{}", gs.score).as_str(),
        (0, 0),
    );
    print_xy(3, 6, Color::AnsiValue(1), "Level", (0, 0));
    print_xy(
        3,
        7,
        Color::AnsiValue(1),
        format!("{}", new_level).as_str(),
        (0, 0),
    );
    print_xy(3, 9, Color::AnsiValue(1), "Lines", (0, 0));
    print_xy(
        3,
        10,
        Color::AnsiValue(1),
        format!("{}", gs.lines).as_str(),
        (0, 0),
    );

    print_xy(3, 12, Color::AnsiValue(1), "Next Piece", (0, 0));
}

///
/// Compares the current board with the next board and draws the differences.
/// Copies changes from the next board to the current board, and then swaps the two boards.
///
fn draw_diff(
    current_board: &mut Board,
    next_board: &mut Board,
    current_piece_color: u8,
    board_offset: (u16, u16),
) {
    for y in 0..next_board.height {
        for x in 0..next_board.width {
            if (current_board.cells[x as usize][y as usize] > 0)
                && (next_board.cells[x as usize][y as usize] == 0)
            {
                print_xy(
                    x as u16 * 2,
                    y as u16,
                    Color::AnsiValue(0),
                    EMPTY_BLOCK,
                    board_offset,
                );
                current_board.cells[x as usize][y as usize] = 0;
            } else if current_board.cells[x as usize][y as usize]
                != next_board.cells[x as usize][y as usize]
            {
                print_xy(
                    x as u16 * 2,
                    y as u16,
                    match next_board.cells[x as usize][y as usize] {
                        0 => Color::AnsiValue(0),
                        254 => Color::AnsiValue(7),
                        255 => Color::AnsiValue(current_piece_color),
                        _ => Color::AnsiValue(next_board.cells[x as usize][y as usize]),
                    },
                    BLOCK,
                    board_offset,
                );
                current_board.cells[x as usize][y as usize] =
                    next_board.cells[x as usize][y as usize];
            }
        }
    }
}
