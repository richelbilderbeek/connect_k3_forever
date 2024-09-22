/// Histroy of the band

pub struct Band {

}

impl Band {
    #[cfg(test)]
    /// @param player_index the player's index:
    /// - 0: player 1, red hair
    /// - 1: player 2, black har
    /// - 2: player 3, blonde hair
    /// @param singer_index the index of the player.
    /// - For the singer with red hair:
    ///   - 0: Karen
    ///   - 1: Hanne
    /// - For the singer with black hair:
    ///   - 0: Kristel
    ///   - 1: Marthe
    /// - For the singer with blonde hair:
    ///   - 0: Kathleen
    ///   - 1: Josje
    ///   - 2: Klaasje
    ///   - 3: Julia
    pub fn get_name(&self, player_index: i8, singer_index: i8) -> String {
        assert!(player_index >= 0);
        assert!(player_index < 3);
        assert!(singer_index >= 0);
        if player_index == 0 { // Red hair
            assert!(singer_index < 2);
            if singer_index == 0 {
                return String::from("karen");
            }
            assert_eq!(singer_index, 1);
            return String::from("hanne")
        }
        if player_index == 1 { // Black
            assert!(singer_index < 2);
            if singer_index == 0 {
                return String::from("kristel");
            }
            assert_eq!(singer_index, 1);
            return String::from("marthe")
        }
        assert_eq!(player_index, 2);
        assert!(singer_index < 4);
        if singer_index == 0 {
            return String::from("kathleen");
        }
        else if singer_index == 1 {
            return String::from("josje");
        }
        else if singer_index == 2 {
            return String::from("klaasje");
        }
        assert_eq!(singer_index, 3);
        return String::from("julia")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_create() {
        let _assets = Band{};
    }
    #[test]
    fn test_can_get_name() {
        let band = Band{};
        assert!(!band.get_name(0, 0).is_empty());
    }
    #[test]
    fn test_names_are_correct() {
        let band = Band{};
        assert!(!band.get_name(0, 0).is_empty());
        assert_eq!(band.get_name(0, 0), "karen");
        assert_eq!(band.get_name(0, 1), "hanne");
        assert_eq!(band.get_name(1, 0), "kristel");
        assert_eq!(band.get_name(1, 1), "marthe");
        assert_eq!(band.get_name(2, 0), "kathleen");
        assert_eq!(band.get_name(2, 1), "josje");
        assert_eq!(band.get_name(2, 2), "klaasje");
        assert_eq!(band.get_name(2, 3), "julia");
    }
}
