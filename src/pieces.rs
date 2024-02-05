#[derive(Clone)]
pub struct Square {
    pub x: i32,
    pub y: i32,
}

pub type PieceView = [Square; 4];
#[derive(Clone)]
pub enum Orientation {
    Up,
    Right,
    Down,
    Left,
}
#[derive(Clone)]
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
    Piece {
        color: 1,
        orientation: Orientation::Up,
        up: [
            Square { x: 0, y: 0 },
            Square { x: 1, y: 0 },
            Square { x: 0, y: 1 },
            Square { x: 1, y: 1 },
        ],
        right: [
            Square { x: 0, y: 0 },
            Square { x: 1, y: 0 },
            Square { x: 0, y: 1 },
            Square { x: 1, y: 1 },
        ],
        down: [
            Square { x: 0, y: 0 },
            Square { x: 1, y: 0 },
            Square { x: 0, y: 1 },
            Square { x: 1, y: 1 },
        ],
        left: [
            Square { x: 0, y: 0 },
            Square { x: 1, y: 0 },
            Square { x: 0, y: 1 },
            Square { x: 1, y: 1 },
        ],
    },
    Piece {
        color: 2,
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
        color: 3,
        orientation: Orientation::Up,
        up: [
            Square { x: 0, y: -1 },
            Square { x: 0, y: 0 },
            Square { x: 0, y: 1 },
            Square { x: 0, y: 2 },
        ],
        right: [
            Square { x: -1, y: 0 },
            Square { x: 0, y: 0 },
            Square { x: 1, y: 0 },
            Square { x: 2, y: 0 },
        ],
        down: [
            Square { x: 0, y: -1 },
            Square { x: 0, y: 0 },
            Square { x: 0, y: 1 },
            Square { x: 0, y: 2 },
        ],
        left: [
            Square { x: -1, y: 0 },
            Square { x: 0, y: 0 },
            Square { x: 1, y: 0 },
            Square { x: 2, y: 0 },
        ],
    },
    Piece {
        color: 4,
        orientation: Orientation::Up,
        up: [
            Square { x: 0, y: -1 },
            Square { x: 0, y: 0 },
            Square { x: 0, y: 1 },
            Square { x: 1, y: 1 },
        ],
        right: [
            Square { x: 0, y: 0 },
            Square { x: 1, y: 0 },
            Square { x: -1, y: 0 },
            Square { x: -1, y: 1 },
        ],
        down: [
            Square { x: -1, y: -1 },
            Square { x: 0, y: -1 },
            Square { x: 0, y: 0 },
            Square { x: 0, y: 1 },
        ],
        left: [
            Square { x: -1, y: 0 },
            Square { x: 0, y: 0 },
            Square { x: 1, y: 0 },
            Square { x: 1, y: -1 },
        ],
    },
    Piece {
        color: 5,
        orientation: Orientation::Up,
        up: [
            Square { x: 0, y: -1 },
            Square { x: 0, y: 0 },
            Square { x: 0, y: 1 },
            Square { x: -1, y: 1 },
        ],
        right: [
            Square { x: 0, y: 0 },
            Square { x: 1, y: 0 },
            Square { x: -1, y: 0 },
            Square { x: -1, y: -1 },
        ],
        down: [
            Square { x: 1, y: -1 },
            Square { x: 0, y: -1 },
            Square { x: 0, y: 0 },
            Square { x: 0, y: 1 },
        ],
        left: [
            Square { x: -1, y: 0 },
            Square { x: 0, y: 0 },
            Square { x: 1, y: 0 },
            Square { x: 1, y: 1 },
        ],
    },
    Piece {
        color: 6,
        orientation: Orientation::Up,
        up: [
            Square { x: 0, y: -1 },
            Square { x: 0, y: 0 },
            Square { x: 1, y: 0 },
            Square { x: 1, y: 1 },
        ],
        right: [
            Square { x: -1, y: 1 },
            Square { x: 0, y: 1 },
            Square { x: 0, y: 0 },
            Square { x: 1, y: 0 },
        ],
        down: [
            Square { x: 0, y: -1 },
            Square { x: 0, y: 0 },
            Square { x: 1, y: 0 },
            Square { x: 1, y: 1 },
        ],
        left: [
            Square { x: -1, y: 1 },
            Square { x: 0, y: 1 },
            Square { x: 0, y: 0 },
            Square { x: 1, y: 0 },
        ],
    },
    Piece {
        color: 8,
        orientation: Orientation::Up,
        up: [
            Square { x: 0, y: 1 },
            Square { x: 0, y: 0 },
            Square { x: 1, y: 0 },
            Square { x: 1, y: -1 },
        ],
        right: [
            Square { x: -1, y: -1 },
            Square { x: 0, y: -1 },
            Square { x: 0, y: 0 },
            Square { x: 1, y: 0 },
        ],
        down: [
            Square { x: 0, y: 1 },
            Square { x: 0, y: 0 },
            Square { x: 1, y: 0 },
            Square { x: 1, y: -1 },
        ],
        left: [
            Square { x: -1, y: -1 },
            Square { x: 0, y: -1 },
            Square { x: 0, y: 0 },
            Square { x: 1, y: 0 },
        ],
    },
];
