

#[cfg(test)]
mod tests {


    #[cfg(test)]
    mod tests {



        #[test]
        fn test_name() {
            let mut player = crate::Player::new("Marcus".to_string(), "Car".to_string());
            player.set_username("John".to_string());
            assert_eq!("John", player.username);
        }

        #[test]
        fn test_piece() {
            let mut player = crate::Player::new("John".to_string(), "X".to_string());
            player.set_piece("Hat".to_string());
            assert_eq!("Hat", player.piece);
        }

        #[test]
        fn test_position() {
            let mut player = crate::Player::new("John".to_string(), "X".to_string());
            player.set_position(5);
            assert_eq!(5, player.position);
        }


    }
}