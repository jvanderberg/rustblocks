#[cfg(test)]
mod test {
    use std::{cell::RefCell, rc::Rc};

    use crate::{
        gamestate::{Difficulty, GameEvent, GameState, GameStatus},
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

        pub fn add(&self, ev: GameEvent) {
            self.events.borrow_mut().push(ev);
        }

        pub fn get_handler(&self) -> Box<dyn Fn(&GameEvent, &GameState) + '_> {
            let f = |ev: &GameEvent, _gs: &GameState| self.add(ev.clone());
            Box::new(f)
        }
    }

    use crate::gamestate::GameEvent::*;

    #[cfg(test)]
    #[test]
    pub fn test_create() {
        let test_events = TestEvents::new();
        let mut gs = GameState::new(10, 22, false, Difficulty::Easy);

        let handler = test_events.get_handler();
        gs.add_event_handler(&handler);
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
        let test_events = TestEvents::new();
        let mut gs = GameState::new(10, 22, false, Difficulty::Easy);
        let handler = test_events.get_handler();
        gs.add_event_handler(&handler);

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
        let test_events = TestEvents::new();
        let mut gs = GameState::new(10, 22, false, Difficulty::Easy);
        let handler = test_events.get_handler();
        gs.add_event_handler(&handler);
        gs.initialize_board_pieces();

        let current_piece = gs.get_current_piece().clone();

        let orientation = current_piece.piece.orientation;

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
        let test_events = TestEvents::new();
        let mut gs = GameState::new(10, 22, false, Difficulty::Easy);
        let handler = test_events.get_handler();
        gs.add_event_handler(&handler);
        assert_eq!(
            gs.drop(crate::gamestate::DropSpeed::Off),
            false,
            "Piece should not drop before game started"
        );
        gs.start();
        assert_eq!(
            gs.drop(crate::gamestate::DropSpeed::Off),
            true,
            "Piece should not drop before game started"
        );
        let events = test_events.events.borrow();

        assert!(
            events.contains(&PieceChanged),
            "Piece changed event should be generated"
        );
        assert!(
            events
                .as_slice()
                .iter()
                .filter(|e| **e == PieceMoved)
                .count()
                > 10,
            "Piece moved more than ten times"
        );
    }

    #[test]
    pub fn test_game_over() {
        let mut gs = GameState::new(10, 22, false, Difficulty::Easy);
        gs.initialize_board_pieces();
        let test_events = TestEvents::new();
        gs.start();
        let handler = test_events.get_handler();
        gs.add_event_handler(&handler);

        // Drop til you stop
        while gs.drop(crate::gamestate::DropSpeed::Off) {}
        let events = test_events.events.borrow();
        assert!(
            events.contains(&GameOver),
            "Game Over event should be generated"
        );
        assert_eq!(
            *gs.get_status(),
            GameStatus::GameOver,
            "Game should be over"
        );
    }
}
