use crossterm::style::Color;

use crate::print::print_xy;

pub fn calc_score(lines_cleared: i32, lines: i32, score: i32) -> (i32, i32, i32) {
    let new_lines = lines + lines_cleared;
    let new_level = (new_lines / 10) + 1;

    let new_score = score
        + match lines_cleared {
            1 => 100 * new_level,
            2 => 300 * new_level,
            3 => 500 * new_level,
            4 => 800 * new_level,
            _ => 0,
        };

    print_xy(3, 3, Color::AnsiValue(1), "Score", (0, 0));
    print_xy(
        3,
        4,
        Color::AnsiValue(1),
        format!("{}", new_score).as_str(),
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
        format!("{}", new_lines).as_str(),
        (0, 0),
    );

    print_xy(3, 12, Color::AnsiValue(1), "Next Piece", (0, 0));

    (new_lines, new_score, new_level)
}
