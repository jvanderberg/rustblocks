#[cfg(test)]
mod test {
    use std::{cell::RefCell, rc::Rc};

    use crossterm::event;

    use crate::{
        gamestate::{Difficulty, EventHandler, GameEvent, GameState},
        pieces::Orientation,
    };

    #[derive(Clone, Debug)]
    struct TestEvents {
        pub events: Rc<RefCell<Vec<GameEvent>>>,
    }

    impl TestEvents {
        pub fn new() -> TestEvents {
            TestEvents {
                events: Rc::new(RefCell::new(Vec::new())),
            }
        }
    }
    impl EventHandler for TestEvents {
        fn handle_event(&mut self, _: &GameState, ge: &GameEvent) {
            self.events.borrow_mut().push(ge.clone());
        }
        fn clone_boxed(&self) -> Box<dyn EventHandler> {
            Box::new(Clone::clone(self))
        }
    }

    #[cfg(test)]
    #[test]
    pub fn test_create() {
        use crate::gamestate::Difficulty;

        let mut gs = GameState::new(10, 22, false, Difficulty::Easy);
        gs.initialize_board_pieces();

        assert_eq!(
            gs.get_board().width,
            12,
            "Board width should be 12 for Game width 10"
        );
        assert_eq!(
            gs.get_board().height,
            23,
            "Board height should be 23 for Game height 22"
        );
    }
    #[test]
    pub fn test_move() {
        let mut gs = GameState::new(10, 22, false, Difficulty::Easy);
        gs.initialize_board_pieces();
        let test_events = TestEvents::new();
        gs.add_event_handler(Box::new(test_events.clone()));
        gs.start();
        let current_piece = gs.get_current_piece().clone();

        let x = current_piece.x;
        let y = current_piece.y;

        assert_eq!(gs.move_left(), true, "Piece should move left");
        assert_eq!(gs.move_right(), true, "Piece should move right");
        assert_eq!(gs.move_down(), true, "Piece should move down");
        let current_piece = gs.get_current_piece().clone();
        assert_eq!(current_piece.x, x, "Piece x is same after right and left");
        assert_eq!(current_piece.y, y + 1, "Piece y is incremented after down");
        let events = test_events.events.borrow();
        assert_eq!(events.len(), 4, "3 events should be generated");
        assert_eq!(
            events.as_slice(),
            vec![
                GameEvent::GameStarted,
                GameEvent::PieceMoved,
                GameEvent::PieceMoved,
                GameEvent::PieceMoved
            ],
            "Events should be generated"
        );
    }

    #[test]
    pub fn test_rotate() {
        let mut gs = GameState::new(10, 22, false, Difficulty::Easy);
        gs.initialize_board_pieces();

        let current_piece = gs.get_current_piece().clone();

        let orientation = current_piece.piece.orientation;

        assert_eq!(
            gs.rotate_right(),
            false,
            "Piece should not rotate right when game not started"
        );
        gs.start();
        assert_eq!(gs.rotate_right(), true, "Piece should rotate right");
        assert_eq!(
            gs.get_current_piece().piece.orientation,
            Orientation::Right, // Orientation::Up,
            "Piece is Right after single rotate"
        );
        assert_eq!(gs.rotate_right(), true, "Piece should rotate right");
        assert_eq!(gs.rotate_right(), true, "Piece should rotate right");
        assert_eq!(gs.rotate_right(), true, "Piece should rotate right");

        assert_eq!(
            gs.get_current_piece().piece.orientation,
            orientation, // Orientation::Up,
            "Piece is up after 4 rotations"
        );
    }

    #[test]
    pub fn drop() {
        let mut gs = GameState::new(10, 22, false, Difficulty::Easy);
        gs.initialize_board_pieces();

        assert_eq!(
            gs.drop(),
            false,
            "Piece should not drop before game started"
        );
    }
}
