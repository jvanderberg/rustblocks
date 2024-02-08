#[derive(Clone, PartialEq)]
pub struct Square {
    pub x: i32,
    pub y: i32,
}

pub const BLOCK: &str = "\u{2588}\u{2588}";
pub const EMPTY_BLOCK: &str = "  ";
pub type PieceView = [Square; 4];
#[derive(Clone, PartialEq)]
pub enum Orientation {
    Up,
    Right,
    Down,
    Left,
}
#[derive(Clone, PartialEq)]
pub struct Piece {
    pub color: u8,
    pub orientation: Orientation,
    up: PieceView,
    right: PieceView,
    down: PieceView,
    left: PieceView,
}

impl Piece {
    pub fn view(&self) -> &PieceView {
        match self.orientation {
            Orientation::Up => &self.up,
            Orientation::Right => &self.right,
            Orientation::Down => &self.down,
            Orientation::Left => &self.left,
        }
    }

    pub fn rotate_left(&mut self) {
        self.orientation = match self.orientation {
            Orientation::Up => Orientation::Left,
            Orientation::Right => Orientation::Up,
            Orientation::Down => Orientation::Right,
            Orientation::Left => Orientation::Down,
        };
    }

    pub fn rotate_right(&mut self) {
        self.orientation = match self.orientation {
            Orientation::Up => Orientation::Right,
            Orientation::Right => Orientation::Down,
            Orientation::Down => Orientation::Left,
            Orientation::Left => Orientation::Up,
        };
    }
}

pub const PIECES: [Piece; 7] = [
    // O piece
    Piece {
        color: 11,
        orientation: Orientation::Up,
        up: [
            Square { x: 0, y: -1 },
            Square { x: 1, y: -1 },
            Square { x: 0, y: 0 },
            Square { x: 1, y: 0 },
        ],
        right: [
            Square { x: 0, y: -1 },
            Square { x: 1, y: -1 },
            Square { x: 0, y: 0 },
            Square { x: 1, y: 0 },
        ],
        down: [
            Square { x: 0, y: -1 },
            Square { x: 1, y: -1 },
            Square { x: 0, y: 0 },
            Square { x: 1, y: 0 },
        ],
        left: [
            Square { x: 0, y: -1 },
            Square { x: 1, y: -1 },
            Square { x: 0, y: 0 },
            Square { x: 1, y: 0 },
        ],
    },
    // T Piece
    Piece {
        color: 93,
        orientation: Orientation::Up,
        up: [
            Square { x: 0, y: 0 },
            Square { x: -1, y: 0 },
            Square { x: 1, y: 0 },
            Square { x: 0, y: -1 },
        ],
        right: [
            Square { x: 0, y: 0 },
            Square { x: 1, y: 0 },
            Square { x: 0, y: -1 },
            Square { x: 0, y: 1 },
        ],
        down: [
            Square { x: 0, y: 0 },
            Square { x: -1, y: 0 },
            Square { x: 1, y: 0 },
            Square { x: 0, y: 1 },
        ],
        left: [
            Square { x: 0, y: 0 },
            Square { x: -1, y: 0 },
            Square { x: 0, y: -1 },
            Square { x: 0, y: 1 },
        ],
    },
    Piece {
        // I piece
        color: 14,
        orientation: Orientation::Up,
        up: [
            Square { x: -1, y: 0 },
            Square { x: 0, y: 0 },
            Square { x: 1, y: 0 },
            Square { x: 2, y: 0 },
        ],
        right: [
            Square { x: 1, y: -1 },
            Square { x: 1, y: 0 },
            Square { x: 1, y: 1 },
            Square { x: 1, y: 2 },
        ],
        down: [
            Square { x: -1, y: 1 },
            Square { x: 0, y: 1 },
            Square { x: 1, y: 1 },
            Square { x: 2, y: 1 },
        ],
        left: [
            Square { x: 0, y: -1 },
            Square { x: 0, y: 0 },
            Square { x: 0, y: 1 },
            Square { x: 0, y: 2 },
        ],
    },
    Piece {
        // L piece
        color: 208,
        orientation: Orientation::Up,
        up: [
            Square { x: -1, y: 0 },
            Square { x: 0, y: 0 },
            Square { x: 1, y: 0 },
            Square { x: 1, y: -1 },
        ],
        right: [
            Square { x: 0, y: -1 },
            Square { x: 0, y: 0 },
            Square { x: 0, y: 1 },
            Square { x: 1, y: 1 },
        ],
        down: [
            Square { x: 0, y: 0 },
            Square { x: 1, y: 0 },
            Square { x: -1, y: 0 },
            Square { x: -1, y: 1 },
        ],
        left: [
            Square { x: -1, y: -1 },
            Square { x: 0, y: -1 },
            Square { x: 0, y: 0 },
            Square { x: 0, y: 1 },
        ],
    },
    Piece {
        // J piece
        color: 21,
        orientation: Orientation::Up,
        up: [
            Square { x: 0, y: 0 },
            Square { x: 1, y: 0 },
            Square { x: -1, y: 0 },
            Square { x: -1, y: -1 },
        ],
        right: [
            Square { x: 1, y: -1 },
            Square { x: 0, y: -1 },
            Square { x: 0, y: 0 },
            Square { x: 0, y: 1 },
        ],
        down: [
            Square { x: -1, y: 0 },
            Square { x: 0, y: 0 },
            Square { x: 1, y: 0 },
            Square { x: 1, y: 1 },
        ],
        left: [
            Square { x: 0, y: -1 },
            Square { x: 0, y: 0 },
            Square { x: 0, y: 1 },
            Square { x: -1, y: 1 },
        ],
    },
    Piece {
        // S piece
        color: 10,
        orientation: Orientation::Up,
        up: [
            Square { x: -1, y: 0 },
            Square { x: 0, y: 0 },
            Square { x: 0, y: -1 },
            Square { x: 1, y: -1 },
        ],
        right: [
            Square { x: 0, y: -1 },
            Square { x: 0, y: 0 },
            Square { x: 1, y: 0 },
            Square { x: 1, y: 1 },
        ],
        down: [
            Square { x: -1, y: 1 },
            Square { x: 0, y: 1 },
            Square { x: 0, y: 0 },
            Square { x: 1, y: 0 },
        ],
        left: [
            Square { x: -1, y: -1 },
            Square { x: -1, y: 0 },
            Square { x: 0, y: 0 },
            Square { x: 0, y: 1 },
        ],
    },
    Piece {
        // Z piece
        color: 9,
        orientation: Orientation::Up,
        up: [
            Square { x: -1, y: -1 },
            Square { x: 0, y: -1 },
            Square { x: 0, y: 0 },
            Square { x: 1, y: 0 },
        ],
        right: [
            Square { x: 0, y: 1 },
            Square { x: 0, y: 0 },
            Square { x: 1, y: 0 },
            Square { x: 1, y: -1 },
        ],
        down: [
            Square { x: -1, y: 0 },
            Square { x: 0, y: 0 },
            Square { x: 0, y: 1 },
            Square { x: 1, y: 1 },
        ],
        left: [
            Square { x: -1, y: 1 },
            Square { x: -1, y: 0 },
            Square { x: 0, y: 0 },
            Square { x: 0, y: -1 },
        ],
    },
];
