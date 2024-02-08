use crossterm::style::Color;

use crate::{gamestate::GameState, print::print_xy};

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
