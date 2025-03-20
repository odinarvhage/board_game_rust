#[cfg(test)]
mod tests {
    #[test]
    fn user_test_name() {
        let mut player = crate::Player::new("Marcus".to_string(), "Car".to_string());
        player.set_username("John".to_string());
        assert_eq!("John", player.username);
    }

    #[test]
    fn user_test_piece() {
        let mut player = crate::Player::new("John".to_string(), "X".to_string());
        player.set_piece("Hat".to_string());
        assert_eq!("Hat", player.piece);
    }

    #[test]
    fn user_test_position() {
        let mut player = crate::Player::new("John".to_string(), "X".to_string());
        player.set_position(5);
        assert_eq!(5, player.position);
    }

    #[test]
    fn test_new_board() {
        let board = crate::Board::new(100);
        assert_eq!(99, board.board.len() as u32);
        assert_ne!(100, board.board.len() as u32);
    }

    #[test]
    fn test_add_snake_tiles() {
        let mut board = crate::Board::new(100);
        board.add_event_tiles(10, 2);
        let snake_count = board.board.values().filter(|&&x| x == 2).count() as u32;
        assert_eq!(
            10, snake_count,
            "Expected 10 snake tiles, but found {}",
            snake_count
        );
    }
    #[test]
    fn test_add_ladder_tiles() {
        let mut board = crate::Board::new(100);
        board.add_event_tiles(10, 1);
        let ladder_count = board.board.values().filter(|&&x| x == 1).count() as u32;
        assert_eq!(
            10, ladder_count,
            "Expected 10 ladder tiles, but found {}",
            ladder_count
        );
        assert_ne!(
            0, ladder_count,
            "Expected !0 ladder tiles, but found {}",
            ladder_count
        );
    }

    #[test]

    fn test_add_both_event_tiles() {
        let mut board = crate::Board::new(100);
        board.add_event_tiles(10, 1);
        board.add_event_tiles(10, 2);
        assert_eq!(10, board.board.values().filter(|&&x| x == 1).count() as u32);
        assert_eq!(10, board.board.values().filter(|&&x| x == 2).count() as u32);
    }

    #[test]
    fn test_make_board() {
        let board = crate::make_board(100, 10, 10);
        let board_two = crate::Board::new(100);
        assert_eq!(board_two.board.len(), board.board.len());
    }
}
